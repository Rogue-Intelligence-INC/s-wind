#!/bin/bash
# Blondle Batch Processing
# 1. Extract problems from all domains -> JSON
# 2. Generate comprehensive report -> Markdown

set -e
cd "$(dirname "$0")"

echo "═══════════════════════════════════════════════════"
echo "  Blondle Batch Processing"
echo "═══════════════════════════════════════════════════"

echo ""
echo "[1/2] Extracting problems from all domains..."
cargo run --bin blondle_problems --quiet 2>/dev/null | grep -v "^\[Blondle" > BLONDLE_PROBLEMS.json
echo "      → BLONDLE_PROBLEMS.json"

echo ""
echo "[2/2] Generating comprehensive report..."
cargo run --bin blondle_report --quiet 2>/dev/null | grep -v "^\[Blondle" > BLONDLE_KNOWLEDGE_REPORT.md
echo "      → BLONDLE_KNOWLEDGE_REPORT.md"

echo ""
echo "═══════════════════════════════════════════════════"
echo "  Done! Generated:"
echo "    • BLONDLE_PROBLEMS.json    (structured problem set)"
echo "    • BLONDLE_KNOWLEDGE_REPORT.md (readable report)"
echo "═══════════════════════════════════════════════════"
