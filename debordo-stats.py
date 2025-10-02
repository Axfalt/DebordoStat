import random
import matplotlib.pyplot as plt
import numpy as np
import math

def simulate_attack(day=14, attacking=497, drapo=39):
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
    allocated = [x+(round(attacking * 0.025)) for x in allocated if x > 0]
    return allocated

def estimate_probability(day=30, attacking=900, threshold=0.2, nb_drapo=30, iterations=10000):
    hits = 0

    for _ in range(iterations):
        allocated = simulate_attack(day, attacking, nb_drapo)
        if any(x >= threshold * attacking for x in allocated):
            hits += 1
    return hits / iterations

def debordo(day=14, attacking=185,threshold=15, nb_drapo=40, iterations=100000):
    hits = 0

    for _ in range(iterations):
        allocated = simulate_attack(day, attacking, nb_drapo)
        if any(x > threshold  for x in allocated):
            hits += 1
    return hits / iterations

# Example usage
# prob = estimate_probability(targets=30, attacking=800, threshold=0.13, iterations=100000)
# print(f"Estimated probability: {prob*100:.5f}%")

prob = debordo()
print(f"Estimated probability: {prob*100:.5f}%")

# Parameters
# thresholds = np.linspace(560,580, 20)
# probs = [debordo(day=14, attacking=t,threshold=15, nb_drapo=40, iterations=100000) for t in thresholds]

# plt.figure(figsize=(8, 5))
# plt.plot(thresholds, probs, marker='o')
# plt.xlabel('Threshold (fraction of attacks)')
# plt.ylabel('Estimated Probability')
# plt.title('Probability vs Threshold')
# plt.grid(True)
# plt.show()
