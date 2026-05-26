use rand::prelude::*;
use rand::distr::Uniform;
use rand_mt::Mt64;
use rayon::prelude::*;
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, path::Path};
use std::cmp::max;

const CONFIG_PATH: &str = "SimConfig.toml";
struct AttackSimulator {
    rng: Mt64,
    repartition: Vec<f64>,
    normalized: Vec<f64>,
    allocated: Vec<i32>,
}

#[derive(Deserialize, Clone)]
struct Estimation25Config {
    enabled: bool,
    iterations: usize,
    observations: Vec<String>,
}

#[derive(Deserialize)]
struct Root {
    #[serde(rename = "CONFIG")]
    config: SimConfig,
    #[serde(rename = "ESTIMATION25")]
    estimation25: Option<Estimation25Config>,
}
#[derive(Deserialize)]
struct SimConfig {
    defense_range: (i32, i32),
    tdg_interval: (i32, i32),
    min_def: i32,
    nb_drapo: i32,
    day: i32,
    iterations: u32,
    points: u32,
    is_reactor_built: Option<bool>,
}

impl AttackSimulator {
    fn new(targets: usize) -> Self {
        Self {
            rng: Mt64::new(rand::random()),
            repartition: Vec::with_capacity(targets),
            normalized: Vec::with_capacity(targets),
            allocated: Vec::with_capacity(targets),
        }
    }

    /// Simule une attaque selon la logique du jeu
    fn simulate_attack(&mut self, _day: i32, attacking: i32, drapo: i32, targets: usize) -> &[i32] {
        let mut leftover = attacking;

        // Réduction par les drapeaux
        for _ in 0..drapo {
            leftover -= (attacking as f64 * 0.025).round() as i32;
        }

        let flag_bonus = (attacking as f64 * 0.025).round() as i32;

        if leftover <= 0 {
            self.allocated.clear();
            self.allocated.resize(targets, flag_bonus);
            return &self.allocated;
        }

        // Step 1: Poids aléatoires
        self.repartition.clear();
        for _ in 0..targets {
            self.repartition.push(self.rng.random::<f64>());
        }

        // Step 2: Une cible reçoit un boost de +0.3
        let unlucky_index = self.rng.random_range(0..targets);
        self.repartition[unlucky_index] += 0.3;

        // Step 3: Normalisation
        let sum_weights: f64 = self.repartition.iter().sum();
        self.normalized.clear();
        for &w in &self.repartition {
            self.normalized.push(w / sum_weights);
        }

        // Step 4: Allocation des attaques (et arrondi)
        self.allocated.clear();
        let mut allocated_sum = 0;
        for &p in &self.normalized {
            let mut val = (p * leftover as f64).round() as i32;
            val = val.max(0).min(leftover);
            self.allocated.push(val);
            allocated_sum += val;
        }

        // Step 5: Allocation des attaques restantes
        let mut attacking_cache = leftover - allocated_sum;
        while attacking_cache > 0 {
            let idx = self.rng.random_range(0..targets);
            self.allocated[idx] += 1;
            attacking_cache -= 1;
        }

        // Ajout de l'influence des drapeaux
        for x in &mut self.allocated {
            *x += flag_bonus;
        }
        &self.allocated
    }
}

fn debordo_sequential(
    day: i32,
    attacking: i32,
    threshold: i32,
    nb_drapo: i32,
    iterations: u32,
    is_reactor_built: Option<bool>,
) -> f64 {
    let mut hits = 0;
    let mut rng = rand::rng();
    let reactor_damage = Uniform::new_inclusive(100, 250).unwrap();
    let targets = (10 + 2 * ((day - 10).max(0) / 2)) as usize;
    let mut simulator = AttackSimulator::new(targets);
    
    for _ in 0..iterations {
        let real_attacking = if is_reactor_built.unwrap_or(false) {attacking + reactor_damage.sample(&mut rng)} else {attacking};
        let allocated = simulator.simulate_attack(day, real_attacking, nb_drapo, targets);
        if allocated.iter().any(|&x| x > threshold) {
            hits += 1;
        }
    }
    hits as f64 / iterations as f64
}

fn attack_distribution(tdg_min: i32, tdg_max: i32, day: i32) -> HashMap<i32, f64> {
    if tdg_min > tdg_max {
        return HashMap::new();
    }
    let ratio = if day <= 3 { 0.75 } else { 1.1 } as f64;
    let lo = (ratio * (max(1, day - 1) as f64 * 0.75 + 2.5).powi(3)).round() as i32;
    let hi = (ratio * (day as f64 * 0.75 + 3.5).powi(3)).round() as i32;
    let mid = lo as f64 + 0.5 * (hi - lo) as f64;
    let mid_floor = mid.floor() as i32;

    let total_count = (tdg_max - tdg_min + 1) as f64;
    let p = 1.0 / total_count;

    let n_high = if mid_floor < tdg_max {
        (tdg_max - mid_floor) as f64
    } else {
        0.0
    };

    let reroll_prob = n_high * p;

    let mut prob = HashMap::new();
    for i in tdg_min..=tdg_max {
        if i <= mid_floor {
            prob.insert(i, p + reroll_prob * p);
        } else {
            prob.insert(i, reroll_prob * p);
        }
    }

    prob
}

fn get_attack_distribution(
    day: i32,
    tdg_interval: (i32, i32),
    est25_config: Option<&Estimation25Config>,
) -> HashMap<i32, f64> {
    if let Some(est) = est25_config {
        if est.enabled {
            let obs: Vec<estimation25::Observation> = est
                .observations
                .iter()
                .filter_map(|s| match estimation25::parse_obs(s) {
                    Ok(obs) => Some(obs),
                    Err(err) => {
                        eprintln!("[ESTIMATION25] Observation ignorée '{}': {}", s, err);
                        None
                    }
                })
                .collect();

            if obs.is_empty() {
                eprintln!(
                    "[ESTIMATION25] Aucune observation valide, retour au mode classique via tdg_interval."
                );
                return attack_distribution(tdg_interval.0, tdg_interval.1, day);
            }

            let (dist, total_matches) = estimation25::run_estimator(day as i64, est.iterations, &obs);
            if total_matches > 0 {
                return dist.into_iter().map(|(k, v)| (k as i32, v as f64 / total_matches as f64)).collect();
            }

            eprintln!(
                "[ESTIMATION25] 0 match compatible sur {} itérations, retour au mode classique via tdg_interval.",
                est.iterations
            );
        }
    }
    attack_distribution(tdg_interval.0, tdg_interval.1, day)
}

/// Calcule la probabilité de débordement
fn overflow_probability(
    defense: f64,
    prob_dist: &HashMap<i32, f64>,
    min_def: i32,
    nb_drapo: i32,
    day: i32,
    iterations: u32,
    is_reactor_built : Option<bool>,
) -> f64 {

    let mut overflow_prob = 0.0;

    for (&attack, &base_prob) in prob_dist {
        let overflow = attack as f64 - defense;
        if overflow > 0.0 {
            let overflow_int = overflow as i32;
            let success_prob = debordo_sequential(day, overflow_int, min_def, nb_drapo, iterations, is_reactor_built);
            overflow_prob += base_prob * success_prob;
        }
    }

    overflow_prob * 100.0
}

/// Calcule les probabilités de mort pour une plage de défenses
fn calculate_defense_probabilities(
    defense_range: (i32, i32),
    tdg_interval: (i32, i32),
    min_def: i32,
    nb_drapo: i32,
    day: i32,
    iterations: u32,
    points: u32,
    is_reactor_built: Option<bool>,
    est25_config: Option<&Estimation25Config>,
) -> Vec<(f64, f64)> {
    let step = ((defense_range.1 as f64 + 1.0) - defense_range.0 as f64) / points as f64;

    let prob_dist = get_attack_distribution(day, tdg_interval, est25_config);

    (0..points)
        .into_par_iter()
        .map(|i| {
            let defense = defense_range.0 as f64 + i as f64 * step;
            let prob =
                overflow_probability(defense, &prob_dist, min_def, nb_drapo, day, iterations, is_reactor_built);
            println!(
                "Sim {}, Défense: {:.1}, Probabilité de mort: {:.3}%",
                i, defense, prob
            );
            (defense, prob)
        })
        .collect()
}

fn load_config<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let cfg: Root = toml::from_str(&raw)?;
    Ok(cfg)
}

fn main() {
    println!("🦀 Démarrage du calcul ...");
    let start = Instant::now();

    let root = load_config(CONFIG_PATH)
        .expect("SimConfig.toml n'est pas correctement renseigné");
    let config = root.config;
    let est25_config = root.estimation25;

    println!("Paramètres:");
    println!("  - Intervalle TDG: {:?}", config.tdg_interval);
    println!("  - Plage de défense: {:?}", config.defense_range);
    println!("  - Défense minimale: {}", config.min_def);
    println!("  - Nombre de drapeaux: {}", config.nb_drapo);
    println!("  - Jour: {}", config.day);
    println!("  - Itérations: {}", config.iterations);
    println!("  - Points: {}", config.points);
    println!("  - Réacteur construit: {}", config.is_reactor_built.unwrap_or(false));
    println!();

    // Calcul des probabilités
    let results = calculate_defense_probabilities(
        config.defense_range,
        config.tdg_interval,
        config.min_def,
        config.nb_drapo,
        config.day,
        config.iterations,
        config.points,
        config.is_reactor_built,
        est25_config.as_ref(),
    );

    // Tri des résultats par défense
    let mut sorted_results = results;
    let path = "results.txt";
    let output = File::create(path).unwrap();
    sorted_results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    sorted_results.iter().for_each(|r| {
        let _ = writeln!(
            &output,
            "Défense: {:.1}, Probabilité de mort: {:.3}%",
            r.0, r.1
        );
    });

    let duration = start.elapsed();
    println!("\n⏱️  Temps d'exécution: {:.2?}", duration);
    println!("✅ Calcul terminé. Résultats sauvegardés dans '{}'.", path);
}
