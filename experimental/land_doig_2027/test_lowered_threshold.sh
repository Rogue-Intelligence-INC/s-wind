#!/bin/bash
# 调用实验副本，不使用仓库根目录的 swind。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
FORK="$ROOT/fork/target/release/swind-explore-lowered"
if [[ ! -x "$FORK" ]]; then
  echo "先编译副本: cargo build --release --manifest-path $ROOT/fork/Cargo.toml"
  exit 1
fi
KB="$ROOT/fork_kb.json"
rm -f "$KB"
for seq in "52,45,38" "50,47,40" "48,43,35"; do
  "$FORK" explore "$seq" --mode verified --save-pending --kb "$KB" | tail -2
done
python3 - "$KB" << 'PY'
import json, sys
kb = json.load(open(sys.argv[1]))
for d in kb["discovered"]:
    print(d["id"], d["status"], d["equation_tex"])
PY
