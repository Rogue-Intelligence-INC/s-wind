"""Independent covering-array checker and small searches.

A CA(N; t, k, v) is an N by k array on symbols {0, ..., v-1} such that
every set of t columns contains each of the v**t tuples at least once.
"""

from __future__ import annotations

import itertools
import random
import time
from math import comb


def schonheim_lower_bound(t: int, k: int, v: int) -> int:
    """Schönheim bound: N >= ceil(v * N(t-1, k-1, v)), with N(1, *, v) = v.

    This is a mathematical lower bound, not a search result.
    """
    if t <= 0:
        return 1
    if k < t:
        return 1
    if t == 1:
        return v
    child = schonheim_lower_bound(t - 1, k - 1, v)
    return v * child


def trivial_tuple_bound(t: int, v: int) -> int:
    """Each t columns need v**t distinct rows, so N >= v**t."""
    return v**t


def verify_covering(array: list[list[int]], t: int, v: int) -> dict:
    """Check every t-column projection against all v**t tuples."""
    if not array:
        return {"ok": False, "reason": "empty"}
    n_rows = len(array)
    k = len(array[0])
    if any(len(row) != k for row in array):
        return {"ok": False, "reason": "ragged"}
    for r, row in enumerate(array):
        for c, symbol in enumerate(row):
            if not isinstance(symbol, int) or symbol < 0 or symbol >= v:
                return {"ok": False, "reason": "bad_symbol", "row": r, "col": c}

    missing = []
    covered_blocks = 0
    expected = v**t
    for cols in itertools.combinations(range(k), t):
        seen: set[tuple[int, ...]] = set()
        for row in array:
            seen.add(tuple(row[c] for c in cols))
        covered_blocks += 1
        if len(seen) != expected:
            absent = []
            for tup in itertools.product(range(v), repeat=t):
                if tup not in seen:
                    absent.append(tup)
                    if len(absent) >= 3:
                        break
            missing.append(
                {
                    "columns": list(cols),
                    "covered": len(seen),
                    "expected": expected,
                    "absent_examples": absent,
                }
            )
    return {
        "ok": not missing,
        "rows": n_rows,
        "columns": k,
        "strength": t,
        "symbols": v,
        "blocks": covered_blocks,
        "tuples_per_block": expected,
        "missing_blocks": len(missing),
        "missing_examples": missing[:5],
    }


def _binary_pair_ok(a: int, b: int, full: int) -> bool:
    if a == 0 or a == full or b == 0 or b == full:
        return False
    if (a & b) == 0:
        return False
    if (a | b) == full:
        return False
    if (a & ~b & full) == 0:
        return False
    if (~a & b & full) == 0:
        return False
    return True


def exhaustive_binary_strength2(n_rows: int, k: int) -> dict:
    """Enumerate every ordered k-tuple of distinct {0,1}^n_rows columns.

    Complete for these parameters: the domain is finite and every tuple is tested.
    """
    full = (1 << n_rows) - 1
    domain = full + 1
    nodes = 0
    found = None
    count = 0
    t0 = time.perf_counter()

    def columns_of(masks: tuple[int, ...]) -> list[list[int]]:
        rows = []
        for r in range(n_rows):
            rows.append([(mask >> r) & 1 for mask in masks])
        return rows

    if k == 3:
        for a in range(domain):
            for b in range(domain):
                if not _binary_pair_ok(a, b, full):
                    continue
                for c in range(domain):
                    nodes += 1
                    if _binary_pair_ok(a, c, full) and _binary_pair_ok(b, c, full):
                        count += 1
                        if found is None:
                            found = columns_of((a, b, c))
    elif k == 4:
        for a in range(domain):
            for b in range(a + 1, domain):
                if not _binary_pair_ok(a, b, full):
                    continue
                for c in range(b + 1, domain):
                    if not (_binary_pair_ok(a, c, full) and _binary_pair_ok(b, c, full)):
                        continue
                    for d in range(c + 1, domain):
                        nodes += 1
                        if (
                            _binary_pair_ok(a, d, full)
                            and _binary_pair_ok(b, d, full)
                            and _binary_pair_ok(c, d, full)
                        ):
                            count += 1
                            if found is None:
                                found = columns_of((a, b, c, d))
    else:
        raise ValueError("exhaustive scanner is implemented for k=3 and k=4 only")

    return {
        "rows": n_rows,
        "columns": k,
        "symbols": 2,
        "strength": 2,
        "domain": "unordered distinct columns" if k == 4 else "ordered columns, repeats allowed but rejected by the pair test",
        "nodes": nodes,
        "covering_arrays_found": count,
        "seconds": time.perf_counter() - t0,
        "example": found,
        "complete": True,
    }


def load_array(path: str) -> list[list[int]]:
    rows = []
    with open(path) as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            rows.append([int(x) for x in line.replace(",", " ").split()])
    return rows


def coverage_count(array: list[list[int]], v: int, t: int = 2) -> int:
    k = len(array[0])
    seen: set[tuple] = set()
    for row in array:
        for cols in itertools.combinations(range(k), t):
            seen.add((cols, tuple(row[c] for c in cols)))
    return len(seen)


def repair_from_published(
    published: list[list[int]],
    v: int,
    seconds: float,
    seed: int,
) -> dict:
    """Try to delete one row of a known array and repair coverage.

    Partial search. Failure does not prove that N-1 is impossible.
    """
    target_rows = len(published) - 1
    k = len(published[0])
    t = 2
    goal = comb(k, t) * (v**t)
    rng = random.Random(seed)
    t0 = time.perf_counter()
    best_score = -1
    best_array = None
    trials = 0
    mutations = 0
    deadline = t0 + seconds
    per_start = seconds / max(1, len(published))

    while time.perf_counter() < deadline:
        drop = trials % len(published)
        base = [row[:] for i, row in enumerate(published) if i != drop]
        # one random restart per deleted row, then hill-climb until the slice ends
        slice_end = min(deadline, t0 + (trials + 1) * per_start)
        score = coverage_count(base, v, t)
        trials += 1
        stuck = 0
        while time.perf_counter() < slice_end:
            mutations += 1
            r = rng.randrange(target_rows)
            c = rng.randrange(k)
            old = base[r][c]
            base[r][c] = rng.randrange(v)
            new_score = coverage_count(base, v, t)
            if new_score >= score:
                score = new_score
                stuck = 0
                if score > best_score:
                    best_score = score
                    best_array = [row[:] for row in base]
                    if score == goal:
                        return {
                            "improved": True,
                            "rows": target_rows,
                            "columns": k,
                            "symbols": v,
                            "strength": t,
                            "score": score,
                            "goal": goal,
                            "trials": trials,
                            "mutations": mutations,
                            "seconds": time.perf_counter() - t0,
                            "seed": seed,
                            "complete": False,
                            "array": best_array,
                            "dropped_published_row": drop,
                        }
            else:
                base[r][c] = old
                stuck += 1
                if stuck > 400:
                    # jump
                    r2 = rng.randrange(target_rows)
                    c2 = rng.randrange(k)
                    base[r2][c2] = rng.randrange(v)
                    score = coverage_count(base, v, t)
                    stuck = 0
        if score > best_score:
            best_score = score
            best_array = [row[:] for row in base]

    return {
        "improved": False,
        "rows": target_rows,
        "columns": k,
        "symbols": v,
        "strength": t,
        "score": best_score,
        "goal": goal,
        "trials": trials,
        "mutations": mutations,
        "seconds": time.perf_counter() - t0,
        "seed": seed,
        "complete": False,
        "array": None,
        "best_incomplete_kept": best_array is not None,
    }
