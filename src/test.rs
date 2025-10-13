use rand::prelude::*;
use rand_mt::Mt64;
use std::collections::HashMap;

#[derive(Clone)]
struct AttackSimulator {
    rng: Mt64,  // Utilise Mersenne Twister pour cohérence avec Python
}

impl AttackSimulator {
    fn new_with_seed(seed: u64) -> Self {
        Self {
            rng: Mt64::new(seed),
        }
    }

    fn simulate_attack(&mut self, day: i32, attacking: i32, drapo: i32) -> Vec<i32> {
        let targets = 10 + 2 * ((day - 10).max(0) / 2);
        let mut leftover = attacking;

        for _ in 0..drapo {
            leftover -= (attacking as f64 * 0.025).round() as i32;
        }

        let mut repartition: Vec<f64> = (0..targets)
            .map(|_| self.rng.gen::<f64>())
            .collect();

        let unlucky_index = self.rng.gen_range(0..targets as usize);
        repartition[unlucky_index] += 0.3;

        let sum_weights: f64 = repartition.iter().sum();
        let normalized: Vec<f64> = repartition.iter().map(|x| x / sum_weights).collect();

        let mut allocated: Vec<i32> = normalized
            .iter()
            .map(|p| 0.max((p * leftover as f64).round() as i32).min(leftover))
            .collect();

        let mut attacking_cache = leftover - allocated.iter().sum::<i32>();
        while attacking_cache > 0 {
            let idx = self.rng.gen_range(0..targets as usize);
            allocated[idx] += 1;
            attacking_cache -= 1;
        }

        let flag_bonus = (attacking as f64 * 0.025).round() as i32;
        allocated.iter_mut().for_each(|x| *x += flag_bonus);
        allocated
    }
}

fn debordo_test(day: i32, attacking: i32, threshold: i32, nb_drapo: i32, iterations: u32, seed: u64) -> f64 {
    let mut hits = 0;
    for i in 0..iterations {
        let mut simulator = AttackSimulator::new_with_seed(seed + i as u64);
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

fn overflow_probability_test(
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
        let overflow = attack as f64 - defense;
        if overflow > 0.0 {
            let overflow_int = overflow as i32;
            let success_prob = debordo_test(day, overflow_int, min_def, nb_drapo, iterations, 42);
            overflow_prob += base_prob * success_prob;
        }
    }

    overflow_prob * 100.0
}

fn main() {
    println!("🧪 Test de cohérence Rust - Version déterministe");
    println!("=" * 60);

    // Test simple avec un cas connu
    println!("\n1. Test de simulate_attack:");
    let mut sim = AttackSimulator::new_with_seed(42);
    let result = sim.simulate_attack(32, 1000, 40);
    println!("   Résultat simulation (seed=42): {:?}", &result[..5.min(result.len())]);
    println!("   Total: {}, Max: {}", result.iter().sum::<i32>(), result.iter().max().unwrap());

    // Test de distribution d'attaque
    println!("\n2. Test de attack_distribution:");
    let dist = attack_distribution(100, 105);
    println!("   Nombre d'éléments: {}", dist.len());
    let total_prob: f64 = dist.values().sum();
    println!("   Somme des probabilités: {:.6}", total_prob);

    // Test de debordo avec les paramètres Python
    println!("\n3. Test de debordo avec paramètres Python:");
    let test_cases = [
        (32, 1000, 61, 40, 100),
        (32, 1500, 61, 40, 100),
        (32, 2000, 61, 40, 100),
    ];

    for (day, attacking, min_def, nb_drapo, iterations) in test_cases {
        let prob = debordo_test(day, attacking, min_def, nb_drapo, iterations, 42);
        println!("   Paramètres: jour={}, attaque={}, seuil={}", day, attacking, min_def);
        println!("   Probabilité: {:.4} ({:.2}%)", prob, prob * 100.0);
    }

    // Test d'overflow_probability avec une défense
    println!("\n4. Test overflow_probability:");
    let defense = 17800.0;
    let tdg_interval = (19561, 20176);
    let prob = overflow_probability_test(defense, tdg_interval, 61, 40, 32, 100);
    println!("   Défense: {}, Probabilité de mort: {:.2}%", defense, prob);
}
