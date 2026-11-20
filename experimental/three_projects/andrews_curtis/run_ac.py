#!/usr/bin/env python3
"""Andrews–Curtis experiment.

Replays the public training submission with the official verifier, searches
one short training presentation, then runs a bounded search on one scored
presentation. Nothing is uploaded to SAIR.
"""

from __future__ import annotations

import json
import subprocess
import sys
import time
from collections import deque
from pathlib import Path

ROOT = Path("/tmp/Andrews-Curtis")
TOOLS = ROOT / "competition" / "tools"
TRAIN_MANIFEST = ROOT / "competition" / "examples" / "training_manifest.json"
OFFICIAL_MANIFEST = TOOLS / "verifier" / "data" / "manifest.json"
SAMPLE = ROOT / "competition" / "examples" / "sample_submission.txt"
HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"

# Search limits. These are stricter than the contest limits and make the
# search incomplete: a miss is not a proof that no AC sequence exists.
TRAIN_MAX_DEPTH = 8
TRAIN_MAX_TOTAL = 24
TRAIN_MAX_NODES = 2_000_000
SCORED_MAX_DEPTH = 6
SCORED_MAX_TOTAL = 20
SCORED_MAX_NODES = 1_500_000
SCORED_SECONDS = 180.0


def official_cli(manifest: Path, submission: Path) -> dict:
    proc = subprocess.run(
        [
            sys.executable,
            "-m",
            "verifier",
            "--manifest",
            str(manifest),
            "--submission",
            str(submission),
            "--pretty",
        ],
        cwd=str(ROOT),
        env={**dict(**{k: v for k, v in __import__("os").environ.items()}), "PYTHONPATH": str(TOOLS)},
        capture_output=True,
        text=True,
        timeout=120,
    )
    return {
        "returncode": proc.returncode,
        "stdout": proc.stdout,
        "stderr": proc.stderr,
    }


def load_challenge(manifest_path: Path, challenge_id: str) -> dict:
    data = json.loads(manifest_path.read_text())
    for challenge in data["challenges"]:
        if challenge["challenge_id"] == challenge_id:
            challenge = dict(challenge)
            challenge["_limits"] = data["limits"]
            return challenge
    raise KeyError(challenge_id)


def bfs(initial, target, max_depth: int, max_total: int, max_nodes: int, seconds: float):
    sys.path.insert(0, str(TOOLS))
    from verifier.core import apply_move

    start = (tuple(initial[0]), tuple(initial[1]))
    goal = (tuple(target[0]), tuple(target[1]))
    queue = deque([(start, [])])
    seen = {start}
    nodes = 0
    t0 = time.perf_counter()
    deadline = t0 + seconds
    stop = "exhausted"
    while queue:
        state, path = queue.popleft()
        nodes += 1
        if state == goal:
            return {
                "moves": path,
                "nodes": nodes,
                "seconds": time.perf_counter() - t0,
                "seen": len(seen),
                "stop": "solved",
                "max_depth": max_depth,
                "max_total_relator_length": max_total,
                "max_nodes": max_nodes,
            }
        if len(path) >= max_depth:
            continue
        if nodes >= max_nodes or time.perf_counter() > deadline:
            stop = "budget"
            break
        for move in range(14):
            nxt = apply_move(state, move)
            total = len(nxt[0]) + len(nxt[1])
            if total > max_total or nxt in seen:
                continue
            seen.add(nxt)
            queue.append((nxt, path + [move]))
    return {
        "moves": None,
        "nodes": nodes,
        "seconds": time.perf_counter() - t0,
        "seen": len(seen),
        "stop": stop,
        "max_depth": max_depth,
        "max_total_relator_length": max_total,
        "max_nodes": max_nodes,
        "seconds_budget": seconds,
    }


def write_submission(path: Path, challenge_id: str, moves: list[int], comment: str) -> None:
    path.write_text(f"{challenge_id}: {json.dumps(moves)} # {comment}\n")


def main() -> int:
    RESULTS.mkdir(parents=True, exist_ok=True)
    if not OFFICIAL_MANIFEST.is_file():
        print("missing Andrews-Curtis checkout", file=sys.stderr)
        return 2

    # 1. Replay the already published training solution.
    replay = official_cli(TRAIN_MANIFEST, SAMPLE)
    (RESULTS / "sample_submission_verifier.txt").write_text(replay["stdout"] + replay["stderr"])

    # 2. Search a short training presentation ourselves.
    train = load_challenge(TRAIN_MANIFEST, "ms-train-0001")
    train_search = bfs(
        train["initial_relators"],
        train["target_relators"],
        TRAIN_MAX_DEPTH,
        TRAIN_MAX_TOTAL,
        TRAIN_MAX_NODES,
        seconds=120.0,
    )
    train_cli = None
    if train_search["moves"] is not None:
        sub = RESULTS / "ms-train-0001.txt"
        write_submission(sub, "ms-train-0001", train_search["moves"], "found by bounded BFS, not the contest sample")
        train_cli = official_cli(TRAIN_MANIFEST, sub)
        (RESULTS / "ms-train-0001_verifier.txt").write_text(train_cli["stdout"] + train_cli["stderr"])

    # 3. Bounded search on one scored id. Not submitted to SAIR.
    scored = load_challenge(OFFICIAL_MANIFEST, "ac-08491")
    scored_search = bfs(
        scored["initial_relators"],
        scored["target_relators"],
        SCORED_MAX_DEPTH,
        SCORED_MAX_TOTAL,
        SCORED_MAX_NODES,
        SCORED_SECONDS,
    )
    scored_cli = None
    if scored_search["moves"] is not None:
        sub = RESULTS / "ac-08491.txt"
        write_submission(sub, "ac-08491", scored_search["moves"], "local bounded BFS; not uploaded to SAIR")
        scored_cli = official_cli(OFFICIAL_MANIFEST, sub)
        (RESULTS / "ac-08491_verifier.txt").write_text(scored_cli["stdout"] + scored_cli["stderr"])

    def accepted(cli: dict | None) -> bool:
        if not cli or cli["returncode"] != 0:
            return False
        try:
            payload = json.loads(cli["stdout"])
        except json.JSONDecodeError:
            return '"ok": true' in cli["stdout"]
        # pretty output may be a dict or a list of results
        if isinstance(payload, dict):
            results = payload.get("results") or payload.get("solutions") or [payload]
        else:
            results = payload
        if isinstance(results, dict):
            results = [results]
        return any(bool(item.get("ok")) for item in results if isinstance(item, dict))

    report = {
        "uploaded_to_sair": False,
        "repo": "https://github.com/SAIRcompetition/Andrews-Curtis",
        "repo_commit": "a0fd6e6f52d82c93ccc06ab91d81e8fa3678256e",
        "engine": "official verifier replay and move function; search code is this adapter",
        "swind_called": False,
        "replay_public_sample": {
            "submission": "competition/examples/sample_submission.txt",
            "moves_ac": [6, 4, 2, 9, 1, 4, 1],
            "challenge": "ms-train-0160",
            "verifier_returncode": replay["returncode"],
            "accepted": accepted(replay),
            "status": "复现公开训练解",
            "contest_win": False,
        },
        "search_training_ms_train_0001": {
            "initial_relators": train["initial_relators"],
            "search": train_search,
            "verifier_returncode": None if train_cli is None else train_cli["returncode"],
            "accepted": accepted(train_cli),
            "contest_win": False,
            "completeness": "Incomplete whenever stop != solved, and even a solution is only one path under the depth and length caps.",
        },
        "search_scored_ac_08491": {
            "initial_relators": scored["initial_relators"],
            "why_this_id": "shortest total relator length among ac-* rows in the official manifest",
            "search": scored_search,
            "verifier_returncode": None if scored_cli is None else scored_cli["returncode"],
            "accepted": accepted(scored_cli),
            "contest_win": False,
            "completeness": "Depth, word-length and node caps are below the contest limits. Failure is not evidence that the presentation is AC-nontrivial.",
        },
    }
    (RESULTS / "report.json").write_text(json.dumps(report, indent=2))
    print(json.dumps({
        "sample_accepted": report["replay_public_sample"]["accepted"],
        "train_0001_moves": train_search["moves"],
        "train_0001_accepted": report["search_training_ms_train_0001"]["accepted"],
        "train_nodes": train_search["nodes"],
        "scored_moves": scored_search["moves"],
        "scored_stop": scored_search["stop"],
        "scored_nodes": scored_search["nodes"],
        "scored_accepted": report["search_scored_ac_08491"]["accepted"],
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
