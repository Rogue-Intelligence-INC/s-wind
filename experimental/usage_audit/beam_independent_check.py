#!/usr/bin/env python3
"""Recount the declared space in examples/industry_beam_optimize.rs.

Uses the same sections, grades, spacings, and GB 50017 formulas.
Fractions only. Does not call the library and does not search a different model.
"""

from fractions import Fraction

SECTIONS = [
    # name, h, b, tw100, tf100, area100, kgm100, ix, wx100
    ("HN150x75", 150, 75, 500, 700, 1816, 1430, 666, 8880),
    ("HN175x90", 175, 90, 500, 800, 2304, 1810, 1220, 14000),
    ("HN200x100", 200, 100, 550, 800, 2757, 2170, 1880, 18800),
    ("HN250x125", 250, 125, 600, 900, 3787, 2970, 4080, 32600),
    ("HN300x150", 300, 150, 650, 900, 4678, 3670, 7210, 48100),
    ("HN350x175", 350, 175, 700, 1100, 6366, 5000, 13700, 78200),
    ("HN400x200", 400, 200, 800, 1300, 8412, 6600, 23700, 119000),
    ("HN450x200", 450, 200, 900, 1400, 9741, 7650, 33500, 149000),
    ("HN500x200", 500, 200, 1000, 1600, 11420, 8960, 48100, 192000),
    ("HN600x200", 600, 200, 1100, 1700, 13440, 10550, 77600, 259000),
]
GRADES = [("Q235", 420), ("Q355", 445)]
SPACINGS = [1500, 2000, 2500, 3000, 3500, 4000]


def strengths(grade, tf100):
    table = {
        ("Q235", True): (215, 125),
        ("Q235", False): (205, 120),
        ("Q355", True): (305, 175),
        ("Q355", False): (295, 170),
    }
    return table[(grade, tf100 <= 1600)]


def main():
    span = Fraction(6000)
    load_d = Fraction(8)
    load_k = Fraction(11, 2)
    e_mod = Fraction(206000)
    gamma_x = Fraction(21, 20)
    g = Fraction(981, 100)
    feasible = []
    total = 0
    for name, h, _b, tw100, tf100, _area, kgm100, ix_cm4, wx100 in SECTIONS:
        self_knm = Fraction(kgm100, 100) * g / 1000
        wx = Fraction(wx100, 100) * 1000
        ix = Fraction(ix_cm4) * 10000
        hw = Fraction(h * 100 - 2 * tf100, 100)
        tw = Fraction(tw100, 100)
        for grade, price100 in GRADES:
            f, fv = strengths(grade, tf100)
            for sp in SPACINGS:
                total += 1
                sp_m = Fraction(sp, 1000)
                q = load_d * sp_m + Fraction(13, 10) * self_knm
                qk = load_k * sp_m + self_knm
                m = q * span * span / 8
                v = q * span / 2
                sigma = m / (gamma_x * wx)
                tau = v / (hw * tw)
                defl = 5 * qk * span**4 / (384 * e_mod * ix)
                defl_lim = span / 250
                if sigma <= f and tau <= fv and defl <= defl_lim:
                    cost = Fraction(kgm100, 100) * Fraction(price100, 100) / sp_m
                    feasible.append((cost, f"{name} {grade} spacing={sp}mm", sigma / f, defl / defl_lim, tau / fv))
    feasible.sort(key=lambda row: row[0])
    print(f"examined={total} feasible={len(feasible)}")
    for i, (cost, desc, su, du, tu) in enumerate(feasible[:5], 1):
        print(
            f"#{i} {desc} cost={cost.numerator}/{cost.denominator} "
            f"sigma/f={float(su):.3f} defl/lim={float(du):.3f} tau/fv={float(tu):.3f}"
        )
    best = feasible[0]
    print(f"optimum={best[1]} exact={best[0].numerator}/{best[0].denominator}")


if __name__ == "__main__":
    main()
