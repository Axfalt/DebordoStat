use rand::prelude::*;
use rand_mt::Mt64;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
struct AttackSimulator {
    rng: Mt64, // Utilise Mersenne Twister comme Python
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

        // Réduction par les drapeaux (exactement comme en Python)
        for _ in 0..drapo {
            leftover -= (attacking as f64 * 0.025).round() as i32;
        }

        // CORRECTION CRUCIALE: si leftover <= 0, distribution uniforme du flag bonus
        if leftover <= 0 {
            let flag_bonus = (attacking as f64 * 0.025).round() as i32;
            return vec![flag_bonus; targets as usize];
        }

        // Step 1: Poids aléatoires
        let mut repartition: Vec<f64> = (0..targets)
            .map(|_| self.rng.gen::<f64>())
            .collect();

        // Step 2: Une cible reçoit un boost de +0.3
        let unlucky_index = self.rng.gen_range(0..targets as usize);
        repartition[unlucky_index] += 0.3;

        // Step 3: Normalisation
        let sum_weights: f64 = repartition.iter().sum();
        let normalized: Vec<f64> = repartition.iter().map(|x| x / sum_weights).collect();

        // Step 4: Allocation des attaques (arrondi)
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

        // Ajout de l'influence des drapeaux (comme dans le commentaire Python "Remove zeros")
        let flag_bonus = (attacking as f64 * 0.025).round() as i32;
        allocated.iter_mut().for_each(|x| *x += flag_bonus);
        allocated
    }
}

/// Version séquentielle de debordo (comme en Python)
fn debordo_sequential(day: i32, attacking: i32, threshold: i32, nb_drapo: i32, iterations: u32) -> f64 {
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

/// Calcule la distribution d'attaque selon la logique du jeu
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
    defense_range: (f64, f64),
    tdg_interval: (i32, i32),
    min_def: i32,
    nb_drapo: i32,
    day: i32,
    iterations: u32,
    points: usize,
) -> Vec<(f64, f64)> {
    let step = (defense_range.1 - defense_range.0) / (points - 1) as f64;

    (0..points)
        .into_par_iter()
        .map(|i| {
            let defense = defense_range.0 + i as f64 * step;
            let prob = overflow_probability(defense, tdg_interval, min_def, nb_drapo, day, iterations);
            println!("Défense: {:.1}, Probabilité de mort: {:.3}%", defense, prob);
            (defense, prob)
        })
        .collect()
}

fn main() {
    println!("🦀 Démarrage du calcul Rust optimisé...");
    let start = Instant::now();

    // TO CHANGE
    let tdg_interval = (28008, 28909);
    let defense_range = (26950.0, 27050.0 );
    let min_def = 60;
    let nb_drapo = 34;
    let day = 37;
    let iterations = 10000;
    let points = 11;

    println!("Paramètres:");
    println!("  - Intervalle TDG: {:?}", tdg_interval);
    println!("  - Plage de défense: {:?}", defense_range);
    println!("  - Défense minimale: {}", min_def);
    println!("  - Nombre de drapeaux: {}", nb_drapo);
    println!("  - Jour: {}", day);
    println!("  - Itérations: {}", iterations);
    println!("  - Points: {}", points);
    println!();

    // Calcul des probabilités
    let results = calculate_defense_probabilities(
        defense_range,
        tdg_interval,
        min_def,
        nb_drapo,
        day,
        iterations,
        points,
    );

    // Tri des résultats par défense
    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let duration = start.elapsed();
    println!("\n⏱️  Temps d'exécution: {:.2?}", duration);

    // Trouve les défenses avec probabilité < 1%
    let safe_defenses: Vec<_> = sorted_results.iter()
        .filter(|(_, prob)| *prob < 1.0)
        .collect();

    if !safe_defenses.is_empty() {
        println!("  - Défenses \"sûres\" (< 1%): {} valeurs", safe_defenses.len());
        if let Some((def, _)) = safe_defenses.first() {
            println!("    Défense minimale recommandée: {:.1}", def);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_distribution() {
        let dist = attack_distribution(100, 105);
        assert!(!dist.is_empty());

        let total_prob: f64 = dist.values().sum();
        assert!((total_prob - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_simulate_attack() {
        let mut simulator = AttackSimulator::new();
        let result = simulator.simulate_attack(14, 497, 39);
        assert!(!result.is_empty());

        // La somme des attaques allouées doit être cohérente
        let total: i32 = result.iter().sum();
        assert!(total > 0);
    }

    #[test]
    fn test_debordo_parallel() {
        let prob = debordo_sequential(25, 2460, 60, 40, 100);
        assert!(prob >= 0.0 && prob <= 1.0);
    }
}
