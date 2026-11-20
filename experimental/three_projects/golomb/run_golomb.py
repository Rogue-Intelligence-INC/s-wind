#!/usr/bin/env python3
"""Golomb-ruler experiment.

Small orders are enumerated exhaustively from the pigeonhole lower bound
L >= n(n-1)/2. Order 29 is only a short constructive search. It is not a
proof, and a length that is still >= 623 does not change the public upper bound.
"""

from __future__ import annotations

import json
import random
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"
SWIND = Path("/workspace/target/release/swind")
ORDER29_SECONDS = 120.0
ORDER11_SECONDS = 180.0


def verify_ruler(marks: list[int]) -> dict:
    if marks != sorted(marks) or len(set(marks)) != len(marks):
        return {"ok": False, "reason": "not_strictly_increasing"}
    if marks[0] != 0:
        return {"ok": False, "reason": "not_translated_to_zero"}
    diffs = []
    for i in range(len(marks)):
        for j in range(i):
            diffs.append(marks[i] - marks[j])
    unique = len(diffs) == len(set(diffs)) and all(d > 0 for d in diffs)
    reflected = [marks[-1] - x for x in reversed(marks)]
    return {
        "ok": unique,
        "marks": len(marks),
        "length": marks[-1] - marks[0],
        "positive_differences": len(diffs),
        "unique_positive_differences": unique,
        "translated_to_zero": marks[0] == 0,
        "reflection_lex_leq": marks <= reflected,
    }


def pigeon_lower_bound(n: int) -> int:
    """The C(n,2) positive differences are distinct and at most the length."""
    return n * (n - 1) // 2


def first_ruler(n: int, lo: int, hi: int, seconds: float) -> dict:
    """First canonical ruler with length in [lo, hi], or proof none exist in range.

    Canonical: mark 0 at position 0, end at L, second mark <= final gap,
    and on a tie the sequence is lexicographically <= its reflection.
    Every ruler has one such orientation, so an empty range means no ruler at all.
    """
    nodes = 0
    t0 = time.perf_counter()
    deadline = t0 + seconds
    timed_out = False
    found = None
    examined_through = lo - 1
    witness: list[int] | None = None

    def rec_save(marks: list[int], used: int, next_min: int, limit: int) -> bool:
        nonlocal nodes, witness, timed_out
        nodes += 1
        if time.perf_counter() > deadline:
            timed_out = True
            return False
        m = len(marks)
        remaining = n - m
        if next_min + (remaining - 1) > limit:
            return False
        for x in range(next_min, limit):
            used_x = used
            good = True
            for p in marks:
                d = x - p
                if (used_x >> d) & 1:
                    good = False
                    break
                used_x |= 1 << d
            if not good:
                continue
            if m == n - 2:
                end = limit
                # For n=3 the mark being placed is the second mark.
                second = marks[1] if len(marks) >= 2 else x
                if second > end - x:
                    continue
                end_used = used_x
                ok = True
                for p in marks + [x]:
                    d = end - p
                    if (end_used >> d) & 1:
                        ok = False
                        break
                    end_used |= 1 << d
                if not ok:
                    continue
                seq = marks + [x, end]
                if second == end - x:
                    rev = [end - v for v in reversed(seq)]
                    if seq > rev:
                        continue
                witness = seq
                return True
            marks.append(x)
            if rec_save(marks, used_x, x + 1, limit):
                return True
            marks.pop()
            if timed_out:
                return False
        return False

    for length in range(lo, hi + 1):
        if time.perf_counter() > deadline:
            timed_out = True
            break
        if n == 2:
            witness = [0, length]
            found = witness
            examined_through = length
            break
        witness = None
        if rec_save([0], 0, 1, length):
            found = witness
            examined_through = length
            break
        if timed_out:
            # This length was interrupted. Do not count it as fully examined.
            break
        examined_through = length

    return {
        "marks": n,
        "pigeon_lower_bound": lo,
        "search_upper_cap": hi,
        "examined_through": examined_through,
        "found_length": None if found is None else found[-1],
        "ruler": found,
        "nodes": nodes,
        "seconds": time.perf_counter() - t0,
        "timed_out": timed_out,
        "complete_through_examined": not timed_out,
    }


def greedy_ruler(n: int) -> list[int]:
    marks = [0]
    used: set[int] = set()
    while len(marks) < n:
        x = marks[-1] + 1
        while True:
            diffs = [x - p for p in marks]
            if len(set(diffs)) == len(diffs) and all(d not in used for d in diffs):
                marks.append(x)
                used.update(diffs)
                break
            x += 1
    return marks


def random_greedy(n: int, seconds: float, seed: int, length_cap: int | None) -> dict:
    """Place the next mark at a random feasible position near the end.

    Any ruler returned is checked. Beating length_cap would be an upper-bound
    improvement only after the checker passes. This search is not exhaustive.
    """
    rng = random.Random(seed)
    t0 = time.perf_counter()
    best = None
    trials = 0
    while time.perf_counter() - t0 < seconds:
        trials += 1
        marks = [0]
        used: set[int] = set()
        failed = False
        while len(marks) < n:
            start = marks[-1] + 1
            # try a window of candidate positions in random order
            window = list(range(start, start + 40 + n))
            rng.shuffle(window)
            placed = False
            for x in window:
                diffs = [x - p for p in marks]
                if len(set(diffs)) == len(diffs) and all(d not in used for d in diffs):
                    marks.append(x)
                    used.update(diffs)
                    placed = True
                    break
            if not placed:
                # fall back to the first feasible integer, still a real ruler if it finishes
                x = start
                while True:
                    diffs = [x - p for p in marks]
                    if len(set(diffs)) == len(diffs) and all(d not in used for d in diffs):
                        marks.append(x)
                        used.update(diffs)
                        placed = True
                        break
                    x += 1
                    if x > start + 5000:
                        failed = True
                        break
            if failed:
                break
        if failed or len(marks) != n:
            continue
        if best is None or marks[-1] < best[-1]:
            best = marks
            if length_cap is not None and marks[-1] < length_cap:
                break
    return {
        "trials": trials,
        "seconds": time.perf_counter() - t0,
        "seed": seed,
        "best": best,
        "best_length": None if best is None else best[-1],
        "complete": False,
    }


def call_swind(lengths: list[int]) -> dict:
    if not SWIND.is_file():
        return {"called": False, "reason": "binary missing"}
    seq = ",".join(str(x) for x in lengths)
    proc = subprocess.run(
        [str(SWIND), "explore", seq, "--mode", "verified"],
        capture_output=True,
        text=True,
        timeout=60,
    )
    text = proc.stdout
    formula = None
    for line in text.splitlines():
        if line.startswith("Formula:"):
            formula = line.split("Formula:", 1)[1].strip()
    return {
        "called": True,
        "binary": str(SWIND),
        "argv": ["swind", "explore", seq, "--mode", "verified"],
        "returncode": proc.returncode,
        "formula": formula,
        "stdout": text,
        "note": "Sequence fit of lengths this program already proved. Not a Golomb search and not a certificate.",
    }


def main() -> int:
    RESULTS.mkdir(parents=True, exist_ok=True)
    small = []
    for n in range(2, 11):
        lo = pigeon_lower_bound(n)
        # cap above the known region; the search stops at the first ruler
        result = first_ruler(n, lo, lo + 40, seconds=120.0)
        check = verify_ruler(result["ruler"]) if result["ruler"] else {"ok": False}
        result["verify"] = check
        result["optimal_proof"] = (
            result["ruler"] is not None
            and result["examined_through"] == result["found_length"]
            and not result["timed_out"]
            and check.get("ok")
        )
        result["proof_statement"] = (
            f"No canonical ruler of length {lo}..{result['found_length']-1} exists, "
            f"and one of length {result['found_length']} passed the difference check. "
            "Shorter than the pigeonhole bound is impossible."
            if result["optimal_proof"]
            else "No complete optimality proof for this order in this run."
        )
        small.append(result)
        print(f"n={n} length={result['found_length']} proof={result['optimal_proof']} nodes={result['nodes']} {result['seconds']:.2f}s", flush=True)

    order11 = first_ruler(11, pigeon_lower_bound(11), pigeon_lower_bound(11) + 30, ORDER11_SECONDS)
    if order11["ruler"]:
        order11["verify"] = verify_ruler(order11["ruler"])
    order11["optimal_proof"] = bool(
        order11["ruler"]
        and not order11["timed_out"]
        and order11.get("verify", {}).get("ok")
        and order11["examined_through"] == order11["found_length"]
    )
    print(f"n=11 length={order11['found_length']} proof={order11['optimal_proof']} timeout={order11['timed_out']}", flush=True)

    greedy = greedy_ruler(29)
    greedy_check = verify_ruler(greedy)
    bounded = random_greedy(29, ORDER29_SECONDS, seed=20261121, length_cap=623)
    bounded_check = verify_ruler(bounded["best"]) if bounded["best"] else {"ok": False}
    best_len = bounded["best_length"]
    beats_623 = bool(bounded_check.get("ok") and best_len is not None and best_len < 623)

    proved = [row["found_length"] for row in small if row["optimal_proof"]]
    swind = call_swind(proved) if len(proved) >= 4 else {"called": False, "reason": "not enough proved lengths"}
    (RESULTS / "swind_stdout.txt").write_text(swind.get("stdout") or "")

    # public page values fetched 2026-11-21 01:19 Asia/Shanghai, for comparison only
    page_small = {4: 6, 5: 11, 6: 17, 7: 25, 8: 34, 9: 44, 10: 55}
    comparison = []
    for row in small:
        n = row["marks"]
        if n in page_small and row["optimal_proof"]:
            comparison.append({
                "n": n,
                "our_length": row["found_length"],
                "leaps_page_value": page_small[n],
                "match": row["found_length"] == page_small[n],
            })

    report = {
        "engine_role": "S-wind was called only to fit the length sequence after enumeration. The rulers come from this adapter.",
        "swind": {k: v for k, v in swind.items() if k != "stdout"},
        "small_orders": [
            {
                "n": row["marks"],
                "length": row["found_length"],
                "ruler": row["ruler"],
                "nodes": row["nodes"],
                "seconds": row["seconds"],
                "optimal_proof": row["optimal_proof"],
                "verify_ok": row["verify"].get("ok"),
                "proof": row["proof_statement"],
            }
            for row in small
        ],
        "order_11": {
            "length": order11["found_length"],
            "ruler": order11["ruler"],
            "nodes": order11["nodes"],
            "seconds": order11["seconds"],
            "timed_out": order11["timed_out"],
            "optimal_proof": order11["optimal_proof"],
            "verify_ok": order11.get("verify", {}).get("ok", False),
            "examined_through": order11["examined_through"],
        },
        "order_29": {
            "page_lower_bound": 578,
            "page_upper_bound": 623,
            "page_fetched": "2026-11-21",
            "our_pigeon_lower_bound": pigeon_lower_bound(29),
            "our_lower_bound_is_not_578": True,
            "greedy_ruler": greedy,
            "greedy_length": greedy[-1],
            "greedy_verify": greedy_check,
            "bounded_search": {
                "seconds_budget": ORDER29_SECONDS,
                "seed": 20261121,
                "trials": bounded["trials"],
                "seconds": bounded["seconds"],
                "best_length": best_len,
                "verify_ok": bounded_check.get("ok"),
                "beats_published_upper_623": beats_623,
                "complete": False,
            },
            "record_changed": beats_623,
            "optimal": False,
        },
        "page_comparison_for_proved_orders": comparison,
    }
    (RESULTS / "report.json").write_text(json.dumps(report, indent=2))
    print(json.dumps({
        "proved": [(row["n"], row["length"]) for row in report["small_orders"] if row["optimal_proof"]],
        "n11": report["order_11"]["length"],
        "n11_proof": report["order_11"]["optimal_proof"],
        "n29_greedy": greedy[-1],
        "n29_best": best_len,
        "beats_623": beats_623,
        "swind_formula": swind.get("formula"),
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
