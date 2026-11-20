#!/bin/bash
# Reproduce the three small experiments. Official checkouts are clones, not submissions.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"

if [[ ! -d /tmp/ca-tools/.git ]]; then
  git clone --depth 1 https://github.com/chrispbsmolen/ca-tools.git /tmp/ca-tools
fi
if [[ ! -d /tmp/Andrews-Curtis/.git ]]; then
  git clone --depth 1 https://github.com/SAIRcompetition/Andrews-Curtis.git /tmp/Andrews-Curtis
fi

python3 "$ROOT/test_fast.py"
python3 "$ROOT/golomb/run_golomb.py"
python3 "$ROOT/andrews_curtis/run_ac.py"
python3 "$ROOT/covering_array/run_ca.py"
echo "done"
