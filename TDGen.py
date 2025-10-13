import csv
import math
import random
from typing import List, Tuple, Dict

# ================== CONFIG  ==================
OPT_FEATURE_ATTACKS = "normal"  # 'normal' | 'hard' | 'easy'
OPT_MODIFIER_ESTIM_OFFSET_MIN = 15
OPT_MODIFIER_ESTIM_OFFSET_MAX = 36
OPT_MODIFIER_ESTIM_INITIAL_SHIFT = 10  # <-- shift = 10 (important)
OPT_MODIFIER_ESTIM_VARIANCE = 48
OPT_MODIFIER_ESTIM_SPREAD = 10  # WT spread
OPT_MODIFIER_WT_THRESHOLD = 33  # visibility threshold
# ========================================================================

WT_STEPS = [33, 38, 42, 46, 50, 54, 58, 63, 67, 71, 75, 79, 83, 88, 92, 96, 100]


def php_round_half_up(x: float) -> int:
    return int(math.floor(x + 0.5))


def mt_rand_inclusive(rnd: random.Random, a: int, b: int) -> int:
    if a > b:
        a, b = b, a
    return rnd.randint(a, b)


def chance_php(rnd: random.Random, c: float, cap_min: float = 0.0, cap_max: float = 1.0) -> bool:
    c = max(cap_min, min(c, cap_max))
    if c >= 1.0:
        return True
    if c <= 0.0:
        return False
    # emulate: mt_rand(0,99) < 100.0 * c
    return mt_rand_inclusive(rnd, 0, 99) < (100.0 * c)


def deshift(value: int, bound_min: int, bound_max: int, off_min_ref: float, off_max_ref: float)\
        -> Tuple[float, float, bool]:
    off_min = off_min_ref
    off_max = off_max_ref
    bmin = (value - bound_min) / value
    bmax = (bound_max - value) / value
    changed = False
    if off_min > bmin:
        off_max += (off_min - bmin)
        off_min = bmin
        changed = True
    elif off_max > bmax:
        off_min += (off_max - bmax)
        off_max = bmax
        changed = True
    return off_min, off_max, changed


def prepare_day(day: int, seed: int) -> Dict:
    rnd = random.Random(seed)

    const_ratio_base = 0.5
    const_ratio_low = 0.75

    if OPT_FEATURE_ATTACKS == "hard":
        max_ratio = 3.1
    elif OPT_FEATURE_ATTACKS == "easy":
        max_ratio = const_ratio_low
    else:
        max_ratio = 1.1

    ratio_min = const_ratio_low if day <= 3 else max_ratio
    ratio_max = (const_ratio_base if day <= 1 else const_ratio_low) if day <= 3 else max_ratio

    min_raw = php_round_half_up(ratio_min * pow(max(1, day - 1) * 0.75 + 2.5, 3))
    max_raw = php_round_half_up(ratio_max * pow(day * 0.75 + 3.5, 3))

    value = mt_rand_inclusive(rnd, min_raw, max_raw)
    if value > (min_raw + 0.5 * (max_raw - min_raw)):
        value = mt_rand_inclusive(rnd, min_raw, max_raw)

    if day <= 15:
        factor = 1.0
    elif day <= 20:
        factor = 0.75
    elif day <= 30:
        factor = 0.5
    elif day <= 40:
        factor = 0.25
    else:
        factor = 0.15

    off_min = mt_rand_inclusive(
        rnd,
        php_round_half_up(factor * (OPT_MODIFIER_ESTIM_OFFSET_MIN - OPT_MODIFIER_ESTIM_INITIAL_SHIFT)),
        php_round_half_up(factor * (OPT_MODIFIER_ESTIM_OFFSET_MAX - OPT_MODIFIER_ESTIM_INITIAL_SHIFT)),
    )
    off_max = php_round_half_up(factor * (OPT_MODIFIER_ESTIM_VARIANCE - 2 * OPT_MODIFIER_ESTIM_INITIAL_SHIFT)) - off_min

    # Initial shift (non-zero)
    shift_min = mt_rand_inclusive(rnd, 0, php_round_half_up(OPT_MODIFIER_ESTIM_INITIAL_SHIFT * factor * 100)) / 10000.0
    shift_max = (factor * OPT_MODIFIER_ESTIM_INITIAL_SHIFT / 100.0) - shift_min

    # Rebase shifts
    smin, smax, _ = deshift(value, min_raw, max_raw, shift_min, shift_max)

    target_min = php_round_half_up(value - (value * smin))
    target_max = php_round_half_up(value + (value * smax))

    # Initial rebound on offsets
    o1_prc = off_min / 100.0
    o2_prc = off_max / 100.0
    o1_prc, o2_prc, c1 = deshift(target_min, min_raw, max_raw, o1_prc, o2_prc)
    o1_prc, o2_prc, c2 = deshift(target_max, min_raw, max_raw, o1_prc, o2_prc)
    if c1 or c2:
        off_min = php_round_half_up(o1_prc * 100.0)
        off_max = php_round_half_up(o2_prc * 100.0)
        protect = 3 if day <= 30 else 1
        if off_min < protect:
            off_max -= (protect - off_min)
            off_min = protect
        elif off_max < protect:
            off_min -= (protect - off_max)
            off_max = protect

    return {
        "seed": seed,
        "day": day,
        "min_raw": min_raw,
        "max_raw": max_raw,
        "value": value,  # <-- actual attack draw
        "target_min": target_min,
        "target_max": target_max,
        "off_min": float(off_min),
        "off_max": float(off_max),
        "factor": factor,
    }


def calculate_offsets_progression(seed: int, off_min_start: float, off_max_start: float, min_spread: float):
    """
    Returns list of (off_min, off_max) after k rounds, for k = 1..24

    """
    rnd = random.Random(seed)
    o1 = float(off_min_start)
    o2 = float(off_max_start)
    out = []
    for i in range(24):
        spendable = (max(0.0, o1) + max(0.0, o2)) / (24 - i)

        def calc_next():
            lo = int(math.floor(spendable * 250))
            hi = int(math.floor(spendable * 1000))
            if hi < lo:
                hi = lo
            return mt_rand_inclusive(rnd, lo, hi) / 1000.0

        if (o1 + o2) > min_spread:
            total = (o1 + o2)
            inc_min = chance_php(rnd, (o1 / total) if total > 0 else 0.0)
            alter = calc_next()
            if chance_php(rnd, 0.25):
                alterMax = calc_next()
                o1 = max(0.0, o1 - alter)
                o2 = max(0.0, o2 - alterMax)
            else:
                if inc_min and o1 > 0:
                    o1 = max(0.0, o1 - alter)
                else:
                    o2 = max(0.0, o2 - alter)

        out.append((o1, o2))
    return out


def run_one_series(day: int, seed: int):
    prep = prepare_day(day, seed)
    # WT reseeds with estimation seed before reductions; min_spread = SPREAD - INITIAL_SHIFT
    min_spread = OPT_MODIFIER_ESTIM_SPREAD - OPT_MODIFIER_ESTIM_INITIAL_SHIFT  # with shift=10 -> 0
    offsets = calculate_offsets_progression(seed, prep["off_min"], prep["off_max"], min_spread)

    series: Dict[int, str] = {}
    for idx, (o1, o2) in enumerate(offsets, start=1):  # citizens 1..24
        quality_pct = php_round_half_up(min(idx / 24.0, 1.0) * 100)
        if quality_pct >= OPT_MODIFIER_WT_THRESHOLD:
            mn = php_round_half_up(prep["target_min"] * (1 - o1 / 100.0))
            mx = php_round_half_up(prep["target_max"] * (1 + o2 / 100.0))
            series[quality_pct] = f"{mn}-{mx}"

    return series, prep["value"]


def generate_csv(day: int, runs: int, filename: str):
    with open(filename, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["Run", "Seed"] + [f"{s}%" for s in WT_STEPS] + ["AttackValue"])
        for r in range(1, runs + 1):
            seed = random.randint(1, 2 ** 31 - 1)
            series, attack_value = run_one_series(day, seed)

            row = [r, seed]
            last_val = "-"
            for s in WT_STEPS:
                if s in series:
                    last_val = series[s]
                row.append(last_val)
            row.append(attack_value)
            writer.writerow(row)


if __name__ == "__main__":
    try:
        day = int(input("Enter the day number: ").strip())
        runs = int(input("Enter number of runs: ").strip())
        filename = input("Enter output CSV filename: ").strip()
        if not filename.lower().endswith(".csv"):
            filename += ".csv"
        generate_csv(day, runs, filename)
        print(f"Saved as {filename}")
    except Exception as e:
        print(f"Error: {e}")