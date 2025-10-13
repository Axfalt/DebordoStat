use rand::prelude::*;
use rand_mt::Mt64;
use std::collections::HashMap;

#[derive(Clone)]
struct AttackSimulator {
    rng: Mt64,
}

impl AttackSimulator {
    fn new() -> Self {
        Self {
            rng: Mt64::new(rand::random()),
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

    println!("Distribution d'attaque:");
    for (&attack, &base_prob) in &prob_dist {
        let overflow = attack as f64 - defense;
        if overflow > 0.0 {
            let overflow_int = overflow as i32;
            let success_prob = debordo_sequential(day, overflow_int, min_def, nb_drapo, iterations);
            println!("  Attaque: {}, Overflow: {}, BaseProb: {:.6}, SuccessProb: {:.6}",
                     attack, overflow_int, base_prob, success_prob);
            overflow_prob += base_prob * success_prob;
        }
    }

    overflow_prob * 100.0
}

fn main() {
    println!("🧪 Test de debug overflow_probability");
    println!("{}", "=".repeat(50));

    // Test avec les mêmes paramètres que votre Python
    let defense = 17800.0;
    let tdg_interval = (19561, 20176);
    let min_def = 61;
    let nb_drapo = 40;
    let day = 32;
    let iterations = 1000;

    println!("Paramètres:");
    println!("  Defense: {}", defense);
    println!("  TDG interval: {:?}", tdg_interval);
    println!("  Min def: {}", min_def);
    println!("  Nb drapo: {}", nb_drapo);
    println!("  Day: {}", day);
    println!("  Iterations: {}", iterations);
    println!();

    let result = overflow_probability(defense, tdg_interval, min_def, nb_drapo, day, iterations);

    println!();
    println!("Résultat Rust: {:.3}%", result);
    println!("Résultat Python attendu: 0.3%");
    println!("Facteur de différence: {:.1}x", result / 0.3);
}
