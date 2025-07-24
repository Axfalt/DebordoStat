import random
import matplotlib.pyplot as plt
import numpy as np

def simulate_attack(targets=16, attacking=100):
    # Step 1: Random weights
    repartition = [random.random() for _ in range(targets)]
    # Step 2: One target gets +0.3 boost
    unlucky_index = random.randint(0, targets-1)
    repartition[unlucky_index] += 0.3
    # Step 3: Normalize
    sum_weights = sum(repartition)
    normalized = [x/sum_weights for x in repartition]
    # Step 4: Allocate attacks (rounded)
    allocated = [max(0, min(attacking, round(p*attacking))) for p in normalized]
    # Step 5: Allocate leftover attacks
    attacking_cache = attacking - sum(allocated)
    while attacking_cache > 0:
        idx = random.randint(0, targets-1)
        allocated[idx] += 1
        attacking_cache -= 1
    # Remove zeros (not strictly necessary for probability)
    allocated = [x for x in allocated if x > 0]
    return allocated

def estimate_probability(targets=16, attacking=100, threshold=0.2, nb_drapo=39, iterations=10000):
    hits = 0
    # Remove flag influence
    attacking = round(attacking * (nb_drapo/40))
    for _ in range(iterations):
        allocated = simulate_attack(targets, attacking)
        if any(x >= threshold * attacking for x in allocated):
            hits += 1
    return hits / iterations

# Example usage
# prob = estimate_probability(targets=22, attacking=750, threshold=0.20, iterations=1000000)
# print(f"Estimated probability: {prob*100:.5f}%")

# Parameters
thresholds = np.linspace(0.16,0.21, 50)
probs = [estimate_probability(targets=20, attacking=415, threshold=t, iterations=1000000) for t in thresholds]

plt.figure(figsize=(8, 5))
plt.plot(thresholds, probs, marker='o')
plt.xlabel('Threshold (fraction of attacks)')
plt.ylabel('Estimated Probability')
plt.title('Probability vs Threshold')
plt.grid(True)
plt.show()
