#!/usr/bin/env python3
"""Fast checks. They do not rerun the 10-minute repair or the order-29 search."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent / "covering_array"))
sys.path.insert(0, str(Path(__file__).resolve().parent / "golomb"))

from ca_lib import exhaustive_binary_strength2, schonheim_lower_bound, verify_covering
from run_golomb import first_ruler, pigeon_lower_bound, verify_ruler


def main() -> int:
    assert schonheim_lower_bound(2, 3, 2) == 4
    assert schonheim_lower_bound(2, 5, 3) == 9
    scan = exhaustive_binary_strength2(4, 3)
    assert scan["covering_arrays_found"] > 0
    assert verify_covering(scan["example"], 2, 2)["ok"]
    none = exhaustive_binary_strength2(4, 4)
    assert none["covering_arrays_found"] == 0
    ruler = first_ruler(5, pigeon_lower_bound(5), 11, seconds=10)
    assert ruler["found_length"] == 11
    assert ruler["optimal_proof"] if "optimal_proof" in ruler else verify_ruler(ruler["ruler"])["ok"]
    assert verify_ruler(ruler["ruler"])["ok"]
    assert pigeon_lower_bound(29) == 406
    print("fast tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
