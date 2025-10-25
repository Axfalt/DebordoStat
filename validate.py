import random
import math

def simulate_attack_python(day=14, attacking=497, drapo=39):
    """Version exacte du Python original pour validation"""
    # Step 0 removes flags influence and compute targets
    targets = 10 + 2 * math.floor(max(0, day - 10)/2)
    leftover = attacking
    for i in range(drapo):
        leftover -= round(attacking * 0.025)

    # Step 1: Random weights
    repartition = [random.random() for _ in range(targets)]
    # Step 2: One target gets +0.3 boost
    unlucky_index = random.randint(0, targets-1)
    repartition[unlucky_index] += 0.3
    # Step 3: Normalize
    sum_weights = sum(repartition)
    normalized = [x/sum_weights for x in repartition]
    # Step 4: Allocate attacks (rounded)
    allocated = [max(0, min(leftover, round(p*leftover))) for p in normalized]
    # Step 5: Allocate leftover attacks
    attacking_cache = leftover - sum(allocated)
    while attacking_cache > 0:
        idx = random.randint(0, targets-1)
        allocated[idx] += 1
        attacking_cache -= 1
    # Remove zeros (not strictly necessary for probability)
    allocated = [x + (round(attacking * 0.025)) for x in allocated]
    return allocated

def debordo_python(day=25, attacking=2460, threshold=60, nb_drapo=40, iterations=1000):
    """Version Python pour validation"""
    hits = 0
    for _ in range(iterations):
        allocated = simulate_attack_python(day, attacking, nb_drapo)
        if any(x > threshold for x in allocated):
            hits += 1
    return hits / iterations

def test_consistency():
    """Test de cohérence entre les deux versions"""
    print("🧪 Test de cohérence Python vs Rust")
    print("=" * 50)

    # Test avec différents paramètres
    test_cases = [
        (14, 497, 39, 60, 1000),
        (25, 2460, 40, 60, 1000),
        (32, 1500, 35, 50, 1000),
    ]

    for day, attacking, nb_drapo, threshold, iterations in test_cases:
        print(f"\nTest: jour={day}, attaque={attacking}, drapeaux={nb_drapo}")

        # Test simulation simple
        random.seed(42)  # Pour la reproductibilité
        result1 = simulate_attack_python(day, attacking, nb_drapo)

        random.seed(42)
        result2 = simulate_attack_python(day, attacking, nb_drapo)

        print(f"  Simulation reproductible: {result1 == result2}")
        print(f"  Nombre de cibles: {len(result1)}")
        print(f"  Total attaques: {sum(result1)}")
        print(f"  Max attaque sur cible: {max(result1)}")

        # Test de probabilité (plusieurs runs pour moyenne)
        probs = []
        for seed in range(10):
            random.seed(seed)
            prob = debordo_python(day, attacking, threshold, nb_drapo, iterations)
            probs.append(prob)

        avg_prob = sum(probs) / len(probs)
        print(f"  Probabilité moyenne (10 runs): {avg_prob:.4f}")
        print(f"  Écart-type: {(sum((p - avg_prob)**2 for p in probs) / len(probs))**0.5:.4f}")

if __name__ == "__main__":
    test_consistency()
