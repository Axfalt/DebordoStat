tdg_interval = [12442, 13120]

def attack_distribution(tdg_min, tdg_max):
    mid = tdg_min + 0.5 * (tdg_max - tdg_min)
    possibilities = tdg_max - tdg_min + 1
    factor_high = (2 /( possibilities * 3))*100
    factor_low = (4 / (possibilities * 3))*100

    prob = {
        i: (factor_high if i > mid else factor_low)
        for i in range(tdg_min, tdg_max + 1)
    }
    return prob

