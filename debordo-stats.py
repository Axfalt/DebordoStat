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
    allocated = [x+(round(attacking * 0.025)) for x in allocated]
    return allocated

def debordo(day=25, attacking=2460,threshold=60, nb_drapo=40, iterations=100):
    hits = 0

    for _ in range(iterations):
        allocated = simulate_attack(day, attacking, nb_drapo)
        if any(x > threshold  for x in allocated):
            hits += 1
    return hits / iterations

def attack_distribution(tdg_min, tdg_max):
    # Retourne un dict {tdg: prob} où les tdg <= milieu ont un poids 2, sinon 1.
    if tdg_min > tdg_max:
        return {}

    mid = tdg_min + 0.5 * (tdg_max - tdg_min)
    # dernier entier qui reçoit le poids 2
    high2 = int(math.floor(mid))

    # nombre d'indices totaux et ceux qui ont le poids 2
    total_count = tdg_max - tdg_min + 1
    if high2 < tdg_min:
        count_low = 0
    else:
        count_low = min(high2, tdg_max) - tdg_min + 1

    total_weight = 2 * count_low + (total_count - count_low)
    if total_weight == 0:
        return {}

    prob = {}
    for i in range(tdg_min, tdg_max + 1):
        weight = 2 if i <= high2 else 1
        prob[i] = weight / total_weight

    return prob


def overflow_probability(defense,tdg_interval,min_def,nb_drapo,day,iterations=100):
    prob_dist = attack_distribution(tdg_interval[0], tdg_interval[1])
    overflow_prob = 0

    for attack in prob_dist.keys():
        overflow = attack - defense
        if overflow > 0:
            base_prob = prob_dist[attack]
            success_prob = debordo(day=day, attacking=overflow, threshold=min_def, nb_drapo=nb_drapo, iterations=iterations)
            overflow_prob += base_prob * success_prob
    return overflow_prob*100

def plot_defense_death_probability(defense_range, tdg_interval, min_def=61, nb_drapo=40, day=32, iterations=1000):
    defenses = np.linspace(defense_range[0], defense_range[1], 11)
    probabilities = []

    for defense in defenses:
        prob = overflow_probability(defense, tdg_interval, min_def, nb_drapo, day, iterations)
        print(f"Défense: {defense}, Probabilité de mort: {prob:.3f}%")
        probabilities.append(prob)

    plt.figure(figsize=(10, 6))
    plt.plot(defenses, probabilities, marker='o', markersize=4, linestyle='-', linewidth=2)
    plt.xlabel('Défense')
    plt.ylabel('Probabilité de mort (%)')
    plt.title('Probabilité de mort en fonction de la défense (avec D40 à 60 au jour 27)')
    plt.grid(True, alpha=0.3)


    plt.legend()
    plt.tight_layout()
    plt.show()

# Exemple d'utilisation
# tdg_interval = [19561, 20176]
# defense_range = [17780, 17840]  # Plage de défense à analyser
# plot_defense_death_probability(defense_range, tdg_interval)
#
prob = overflow_probability(17822, (19561, 20176), 61, 40, 32, 1000)
print(f"Python: {prob:.4f}%")
