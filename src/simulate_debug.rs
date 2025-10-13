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

    fn simulate_attack_debug(&mut self, day: i32, attacking: i32, drapo: i32) -> Vec<i32> {
        println!("\n=== SIMULATION DÉTAILLÉE ===");
        println!("Paramètres: day={}, attacking={}, drapo={}", day, attacking, drapo);

        // Step 0: calcul des cibles et suppression de l'influence des drapeaux
        let targets = 10 + 2 * ((day - 10).max(0) / 2);
        println!("Targets calculés: {}", targets);

        let mut leftover = attacking;
        println!("Leftover initial: {}", leftover);

        // Réduction par les drapeaux (exactement comme en Python)
        for i in 0..drapo {
            let reduction = (attacking as f64 * 0.025).round() as i32;
            leftover -= reduction;
            if i < 3 {
                println!("Drapo {}: reduction={}, leftover={}", i+1, reduction, leftover);
            }
        }
        println!("Leftover final après {} drapeaux: {}", drapo, leftover);

        // CORRECTION CRUCIALE: si leftover <= 0, distribution uniforme
        if leftover <= 0 {
            println!("LEFTOVER NÉGATIF OU NUL - Distribution uniforme du flag bonus");
            let flag_bonus = (attacking as f64 * 0.025).round() as i32;
            let result = vec![flag_bonus; targets as usize];

            let final_total: i32 = result.iter().sum();
            let max_attack = result.iter().max().unwrap();
            let min_attack = result.iter().min().unwrap();

            println!("RÉSULTAT FINAL (uniforme):");
            println!("  - Total attaques: {}", final_total);
            println!("  - Attaque max: {}", max_attack);
            println!("  - Attaque min: {}", min_attack);
            println!("  - Moyenne: {:.1}", final_total as f64 / targets as f64);
            println!("=== FIN SIMULATION ===\n");

            return result;
        }

        // Step 1: Poids aléatoires
        let mut repartition: Vec<f64> = (0..targets)
            .map(|_| self.rng.gen::<f64>())
            .collect();
        println!("Quelques poids aléatoires: {:?}", &repartition[0..3.min(targets as usize)]);

        // Step 2: Une cible reçoit un boost de +0.3
        let unlucky_index = self.rng.gen_range(0..targets as usize);
        repartition[unlucky_index] += 0.3;
        println!("Boost +0.3 appliqué à l'index: {}", unlucky_index);

        // Step 3: Normalisation
        let sum_weights: f64 = repartition.iter().sum();
        let normalized: Vec<f64> = repartition.iter().map(|x| x / sum_weights).collect();
        println!("Somme des poids: {:.6}", sum_weights);

        // Step 4: Allocation des attaques (arrondi)
        let mut allocated: Vec<i32> = normalized
            .iter()
            .map(|p| 0.max((p * leftover as f64).round() as i32).min(leftover))
            .collect();
        println!("Allocation initiale - Total: {}", allocated.iter().sum::<i32>());

        // Step 5: Allocation des attaques restantes
        let mut attacking_cache = leftover - allocated.iter().sum::<i32>();
        println!("Attaques restantes à distribuer: {}", attacking_cache);

        while attacking_cache > 0 {
            let idx = self.rng.gen_range(0..targets as usize);
            allocated[idx] += 1;
            attacking_cache -= 1;
        }
        println!("Après distribution du reste - Total: {}", allocated.iter().sum::<i32>());

        // Step 6: Ajout de l'influence des drapeaux (PROBLÉMATIQUE ?)
        let flag_bonus = (attacking as f64 * 0.025).round() as i32;
        println!("Flag bonus à ajouter: {}", flag_bonus);

        allocated.iter_mut().for_each(|x| *x += flag_bonus);

        let final_total: i32 = allocated.iter().sum();
        let max_attack = allocated.iter().max().unwrap();
        let min_attack = allocated.iter().min().unwrap();

        println!("RÉSULTAT FINAL:");
        println!("  - Total attaques: {}", final_total);
        println!("  - Attaque max: {}", max_attack);
        println!("  - Attaque min: {}", min_attack);
        println!("  - Moyenne: {:.1}", final_total as f64 / targets as f64);
        println!("=== FIN SIMULATION ===\n");

        allocated
    }
}

fn main() {
    println!("🔍 Debug détaillé simulate_attack");
    println!("{}", "=".repeat(60));

    let mut sim = AttackSimulator::new();
    let result = sim.simulate_attack_debug(32, 2268, 40);

    println!("Ce résultat devrait correspondre à votre test Python pour les mêmes paramètres.");
}
