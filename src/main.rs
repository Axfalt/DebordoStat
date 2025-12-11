use rand::prelude::*;
use rand_mt::Mt64;
use rayon::prelude::*;
use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, path::Path};

const CONFIG_PATH: &str = "SimConfig.toml";
#[derive(Clone)]
struct AttackSimulator {
    rng: Mt64, // Utilise Mersenne Twister comme Python
}

#[derive(Deserialize)]
struct Root {
    config: SimConfig,
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
}

impl AttackSimulator {
    fn new() -> Self {
        Self {
            rng: Mt64::new(rand::random()),
        }
    }


    fn new_with_seed(seed: u64) -> Self {
        Self {
            rng: Mt64::new(seed),
        }
    }

    /// Simule une attaque selon la logique du jeu
    fn simulate_attack(&mut self, day: i32, attacking: i32, drapo: i32) -> Vec<i32> {
        // Step 0: calcul des cibles et suppression de l'influence des drapeaux
        let targets = 10 + 2 * ((day - 10).max(0) / 2);
        let mut leftover = attacking;

        // Réduction par les drapeaux
        for _ in 0..drapo {
            leftover -= (attacking as f64 * 0.025).round() as i32;
        }

        if leftover <= 0 {
            let flag_bonus = (attacking as f64 * 0.025).round() as i32;
            return vec![flag_bonus; targets as usize];
        }

        // Step 1: Poids aléatoires
        let mut repartition: Vec<f64> = (0..targets).map(|_| self.rng.gen::<f64>()).collect();

        // Step 2: Une cible reçoit un boost de +0.3
        let unlucky_index = self.rng.gen_range(0..targets as usize);
        repartition[unlucky_index] += 0.3;

        // Step 3: Normalisation
        let sum_weights: f64 = repartition.iter().sum();
        let normalized: Vec<f64> = repartition.iter().map(|x| x / sum_weights).collect();

        // Step 4: Allocation des attaques (et arrondi)
        let mut allocated: Vec<i32> = normalized
            .iter()
            .map(|p| 0.max((p * leftover as f64).round() as i32).min(leftover))
            .collect();

        // Step 5: Allocation des attaques restantes
        let mut attacking_cache = leftover - allocated.iter().sum::<i32>();
        while attacking_cache > 0 {
            let idx = self.rng.gen_range(0..targets as usize);
            allocated[idx] += 1;
            attacking_cache -= 1;
        }

        // Ajout de l'influence des drapeaux
        let flag_bonus = (attacking as f64 * 0.025).round() as i32;
        allocated.iter_mut().for_each(|x| *x += flag_bonus);
        allocated
    }
}

fn debordo_sequential(
    day: i32,
    attacking: i32,
    threshold: i32,
    nb_drapo: i32,
    iterations: u32,
) -> f64 {
    let mut hits = 0;
    for _ in 0..iterations {
        let mut simulator = AttackSimulator::new();
        let allocated = simulator.simulate_attack(day, attacking, nb_drapo);
        if allocated.iter().any(|&x| x > threshold) {
            hits += 1;
        }
    }
    hits as f64 / iterations as f64
}

fn attack_distribution(tdg_min: i32, tdg_max: i32) -> HashMap<i32, f64> {
    if tdg_min > tdg_max {
        return HashMap::new();
    }

    let mid = tdg_min as f64 + 0.5 * (tdg_max - tdg_min) as f64;
    let high2 = mid.floor() as i32;

    let total_count = tdg_max - tdg_min + 1;
    let count_low = if high2 < tdg_min {
        0
    } else {
        high2.min(tdg_max) - tdg_min + 1
    };

    let total_weight = 2 * count_low + (total_count - count_low);
    if total_weight == 0 {
        return HashMap::new();
    }

    let mut prob = HashMap::new();
    for i in tdg_min..=tdg_max {
        let weight = if i <= high2 { 2.0 } else { 1.0 };
        prob.insert(i, weight / total_weight as f64);
    }

    prob
}

/// Calcule la probabilité de débordement
fn overflow_probability(
    defense: f64,
    tdg_interval: (i32, i32),
    min_def: i32,
    nb_drapo: i32,
    day: i32,
    iterations: u32,
) -> f64 {
    let prob_dist = attack_distribution(tdg_interval.0, tdg_interval.1);
    let mut overflow_prob = 0.0;

    for (&attack, &base_prob) in &prob_dist {
        // CORRIGÉ: calcul exact comme en Python (attack - defense)
        let overflow = attack as f64 - defense;
        if overflow > 0.0 {
            let overflow_int = overflow as i32;
            // CHANGÉ: utilise la version séquentielle au lieu de parallèle
            let success_prob = debordo_sequential(day, overflow_int, min_def, nb_drapo, iterations);
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
) -> Vec<(f64, f64)> {
    let step = (defense_range.1 as f64 - defense_range.0 as f64) / (points - 1) as f64;

    (0..points)
        .into_par_iter()
        .map(|i| {
            let defense = defense_range.0 as f64 + i as f64 * step;
            let prob =
                overflow_probability(defense, tdg_interval, min_def, nb_drapo, day, iterations);
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
    println!("🦀 Démarrage du calcul Rust optimisé...");
    let start = Instant::now();

    let config = load_config(CONFIG_PATH)
        .expect("SimConfig.toml n'est pas correctement renseigné")
        .config;

    println!("Paramètres:");
    println!("  - Intervalle TDG: {:?}", config.tdg_interval);
    println!("  - Plage de défense: {:?}", config.defense_range);
    println!("  - Défense minimale: {}", config.min_def);
    println!("  - Nombre de drapeaux: {}", config.nb_drapo);
    println!("  - Jour: {}", config.day);
    println!("  - Itérations: {}", config.iterations);
    println!("  - Points: {}", config.points);
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
