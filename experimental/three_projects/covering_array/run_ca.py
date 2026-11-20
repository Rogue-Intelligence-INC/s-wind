#!/usr/bin/env python3
"""Covering-array experiment. Does not call S-wind.

Reproduces small binary arrays by exhaustive search and tries to beat the
published CA(11; 2, 5, 3) by repairing a 10-row deletion. The repair budget
is partial: not finding an array is not an optimality proof.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from ca_lib import (
    exhaustive_binary_strength2,
    load_array,
    repair_from_published,
    schonheim_lower_bound,
    trivial_tuple_bound,
    verify_covering,
)

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"
PUBLISHED_11 = Path("/tmp/ca-tools/table/arrays/CA_11_2_5_3_c33e3aff.txt")
PUBLISHED_4 = Path("/tmp/ca-tools/table/arrays/CA_4_2_3_2.txt")
REPAIR_SECONDS = 600.0
SEED = 20261121


def main() -> int:
    RESULTS.mkdir(parents=True, exist_ok=True)
    if not PUBLISHED_11.is_file() or not PUBLISHED_4.is_file():
        print("missing ca-tools checkout at /tmp/ca-tools", file=sys.stderr)
        return 2

    report: dict = {"engine": "none", "adapter": "experimental/three_projects/covering_array"}

    # --- reproduction: CA(N;2,3,2), table N=4 ---
    bound_3 = max(trivial_tuple_bound(2, 2), schonheim_lower_bound(2, 3, 2))
    scan_3 = exhaustive_binary_strength2(3, 3)
    scan_4 = exhaustive_binary_strength2(4, 3)
    published_4 = load_array(str(PUBLISHED_4))
    check_found = verify_covering(scan_4["example"], 2, 2) if scan_4["example"] else {"ok": False}
    check_pub = verify_covering(published_4, 2, 2)
    report["reproduce_CA_2_3_2"] = {
        "published_N": 4,
        "published_source": "ca-tools ca_table.csv t=2 v=2 k=3, Kleitman-Spencer/Katona and NIST/KSK rows, commit 830809df",
        "math_lower_bound": bound_3,
        "math_lower_bound_reason": "N >= v^t = 4 and Schönheim N >= v^2 = 4",
        "exhaustive_N3": {k: scan_3[k] for k in scan_3 if k != "example"},
        "exhaustive_N4": {k: scan_4[k] for k in scan_4 if k != "example"},
        "lex_or_scan_example_verified": check_found,
        "published_file_verified": check_pub,
        "status": "复现",
        "improved_record": False,
        "optimality": "N>=4 by the tuple lower bound. The N=4 scan found covering arrays, so 4 is optimal. The N=3 scan is extra finite-domain evidence, not required once the bound is 4.",
    }
    (RESULTS / "ca_2_3_2_example.json").write_text(
        json.dumps({"from_exhaustive_scan": scan_4["example"], "published_file": published_4}, indent=2)
    )

    # --- optimal reproduction of the next binary cell, still fully enumerated ---
    bound_4 = max(trivial_tuple_bound(2, 2), schonheim_lower_bound(2, 4, 2))
    scan_4x4 = exhaustive_binary_strength2(4, 4)
    scan_5x4 = exhaustive_binary_strength2(5, 4)
    check5 = verify_covering(scan_5x4["example"], 2, 2) if scan_5x4["example"] else {"ok": False}
    report["reproduce_CA_2_4_2"] = {
        "published_N": 5,
        "published_source": "ca-tools ca_table.csv t=2 v=2 k=4 N=5, Kleitman-Spencer/Katona and CAs::KSK",
        "math_lower_bound": bound_4,
        "math_lower_bound_does_not_rule_out": "N=4 is still above v^t=4, so the bound alone does not prove the table",
        "exhaustive_N4": {k: scan_4x4[k] for k in scan_4x4 if k != "example"},
        "exhaustive_N5_summary": {
            "covering_arrays_found": scan_5x4["covering_arrays_found"],
            "nodes": scan_5x4["nodes"],
            "seconds": scan_5x4["seconds"],
            "complete": True,
            "verified": check5["ok"],
        },
        "status": "复现",
        "improved_record": False,
        "optimality": "Finite exhaustive search: zero CA(4;2,4,2) in the column domain, and at least one CA(5;2,4,2). This is an exhaustive finite-domain proof for this parameter, not a new record.",
    }
    if scan_5x4["example"]:
        (RESULTS / "ca_2_4_2_example.json").write_text(json.dumps(scan_5x4["example"]))

    # --- improvement attempt: published CA(11;2,5,3) ---
    published_11 = load_array(str(PUBLISHED_11))
    check11 = verify_covering(published_11, 2, 3)
    bound_5 = max(trivial_tuple_bound(2, 3), schonheim_lower_bound(2, 5, 3))
    print(f"repair search budget {REPAIR_SECONDS}s", flush=True)
    repair = repair_from_published(published_11, v=3, seconds=REPAIR_SECONDS, seed=SEED)
    repair_public = {k: v for k, v in repair.items() if k != "array"}
    improved = bool(repair.get("improved"))
    if improved:
        check_new = verify_covering(repair["array"], 2, 3)
        (RESULTS / "ca_2_5_3_candidate.json").write_text(json.dumps(repair["array"]))
        improved = bool(check_new["ok"])
        repair_public["independent_verify"] = check_new
    report["attempt_CA_2_5_3"] = {
        "published_N": 11,
        "published_file": str(PUBLISHED_11),
        "published_verified_by_our_checker": check11,
        "math_lower_bound": bound_5,
        "gap": "11 - 9 = 2. N=10 is not ruled out by Schönheim or by v^t.",
        "search": "Delete one row of the published 11-row array, then hill-climb single-cell changes. Seed fixed. Budget is wall time.",
        "repair": repair_public,
        "status": "改进" if improved else "未改进",
        "improved_record": improved,
        "optimality": "未证明。搜索不完备，失败不能当成 N=10 不存在。",
    }

    (RESULTS / "report.json").write_text(json.dumps(report, indent=2))
    print(json.dumps({
        "ca232_found": report["reproduce_CA_2_3_2"]["exhaustive_N4"]["covering_arrays_found"],
        "ca242_N4": report["reproduce_CA_2_4_2"]["exhaustive_N4"]["covering_arrays_found"],
        "ca242_N5": report["reproduce_CA_2_4_2"]["exhaustive_N5_summary"]["covering_arrays_found"],
        "ca253_improved": improved,
        "ca253_best_score": repair_public.get("score"),
        "ca253_goal": repair_public.get("goal"),
        "ca253_seconds": repair_public.get("seconds"),
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
