#!/usr/bin/env python3
"""Recount the same coefficient boxes the audit caller asks the library to search.

This file does not replace the library search. It only lists every a+b*base
inside the stated integer box and the relative error of each hit.
"""

import math

CASES = [
    ("one_plus_two_pi", 1.0 + 2.0 * math.pi, math.pi, -3, 3),
    ("five_plus_sqrt2", 5.0 + math.sqrt(2), math.sqrt(2), -5, 5),
]


def main() -> None:
    for name, target, base, lo, hi in CASES:
        hits = []
        examined = 0
        for a in range(lo, hi + 1):
            for b in range(lo, hi + 1):
                if b == 0:
                    continue
                examined += 1
                value = a + b * base
                error = abs(value - target) / abs(target) * 100.0
                if error < 0.001:
                    hits.append((error, a, b, value))
        hits.sort()
        print(f"{name} examined={examined} hits={len(hits)}")
        for error, a, b, value in hits[:5]:
            print(f"  a={a} b={b} value={value:.16f} error_percent={error:.12f}")
    # The CLI objective in src/bin/swind.rs is the mean of six values k/100,
    # k an integer in 0..=100. This line only restates that six zeros give 0.
    # It does not enumerate the 101^6 grid.
    print("six_zeros_mean=0")


if __name__ == "__main__":
    main()
