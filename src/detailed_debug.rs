use rand::prelude::*;
use rand_mt::Mt64;

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

        // Debug: afficher les paramètres initiaux
        if attacking == 2419 { // Premier cas du debug
            println!("=== DEBUG simulate_attack ===");
            println!("day={}, attacking={}, drapo={}", day, attacking, drapo);
            println!("targets calculés: {}", targets);
        }

        for _ in 0..drapo {
            leftover -= (attacking as f64 * 0.025).round() as i32;
        }

        if attacking == 2419 {
            println!("leftover après drapeaux: {}", leftover);
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

        if attacking == 2419 {
            let max_attack = allocated.iter().max().unwrap();
            let total_attacks: i32 = allocated.iter().sum();
            println!("flag_bonus: {}", flag_bonus);
            println!("total attaques finales: {}", total_attacks);
            println!("attaque max sur une cible: {}", max_attack);
            println!("seuil à dépasser: {}", 61); // CORRIGÉ: utiliser le vrai threshold
            println!("résultat > seuil ? {}", max_attack > &61); // CORRIGÉ: utiliser le vrai threshold
            println!("=== FIN DEBUG ===");
        }

        allocated
    }
}

fn debordo_sequential(day: i32, attacking: i32, threshold: i32, nb_drapo: i32, iterations: u32) -> f64 {
    let mut hits = 0;

    // Debug pour le premier cas
    if attacking == 2419 {
        println!("\n--- TEST debordo_sequential ---");
        println!("Paramètres: day={}, attacking={}, threshold={}, nb_drapo={}, iterations={}",
                 day, attacking, threshold, nb_drapo, iterations);
    }

    for i in 0..iterations {
        let mut simulator = AttackSimulator::new();
        let allocated = simulator.simulate_attack(day, attacking, nb_drapo);
        let success = allocated.iter().any(|&x| x > threshold);
        if success {
            hits += 1;
        }

        // Afficher quelques premiers tests pour debug
        if attacking == 2419 && i < 5 {
            let max_attack = allocated.iter().max().unwrap();
            println!("Iteration {}: max_attack={}, success={}", i, max_attack, success);
        }
    }

    let result = hits as f64 / iterations as f64;

    if attacking == 2419 {
        println!("Résultat final: {}/{} = {:.6}", hits, iterations, result);
        println!("--- FIN TEST ---\n");
    }

    result
}

fn main() {
    println!("🔍 Test de debug détaillé");
    println!("{}", "=".repeat(50));

    // Test avec le VRAI cas problématique du debug original
    // Defense=17800, Attaque=20068, donc Overflow = 20068-17800 = 2268
    let result1 = debordo_sequential(32, 2268, 61, 40, 10);
    println!("Résultat pour 2268 attaques (seuil 61): {:.3}", result1);

    // Test avec un cas plus faible
    let result2 = debordo_sequential(32, 1761, 61, 40, 10);
    println!("Résultat pour 1761 attaques (seuil 61): {:.3}", result2);

    println!("Si ces résultats sont très élevés, il y a un problème dans la logique");
}
