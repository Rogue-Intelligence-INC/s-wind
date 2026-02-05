#!/bin/bash
# S-wind 实验批处理脚本
# Usage: ./experiments.sh [experiment_name]

SWIND="cargo run --release --bin swind --"

cd "$(dirname "$0")"

# 定义实验
declare -A EXPERIMENTS
EXPERIMENTS=(
    # 核心物理
    ["unified"]="22,42,122,442,1722"
    ["alpha"]="137,274,411,548,685"
    ["planck"]="19,38,57,76,95"
    
    # 经典数列
    ["fibonacci"]="1,1,2,3,5,8,13,21,34"
    ["primes"]="2,3,5,7,11,13,17,19,23"
    ["triangular"]="1,3,6,10,15,21,28,36"
    ["squares"]="1,4,9,16,25,36,49,64"
    ["cubes"]="1,8,27,64,125,216,343"
    ["powers2"]="1,2,4,8,16,32,64,128"
    
    # 递推类
    ["lucas"]="2,1,3,4,7,11,18,29"
    ["pell"]="1,2,5,12,29,70,169"
    
    # 自定义测试
    ["custom1"]="1,4,13,40,121"
    ["custom2"]="3,7,15,31,63,127"
)

run_experiment() {
    local name=$1
    local seq=${EXPERIMENTS[$name]}
    
    if [ -z "$seq" ]; then
        echo "❌ Unknown experiment: $name"
        echo "Available: ${!EXPERIMENTS[@]}"
        return 1
    fi
    
    echo "═══════════════════════════════════════════════════"
    echo "  Experiment: $name"
    echo "═══════════════════════════════════════════════════"
    $SWIND detect "$seq"
    echo
}

run_all() {
    echo "╔══════════════════════════════════════════════════╗"
    echo "║        S-wind Full Experiment Suite              ║"
    echo "╚══════════════════════════════════════════════════╝"
    echo
    
    for name in unified alpha planck fibonacci primes triangular squares cubes powers2 lucas pell custom1 custom2; do
        run_experiment "$name"
    done
}

# Main
if [ $# -eq 0 ]; then
    echo "Usage: $0 <experiment|all|list|custom SEQUENCE>"
    echo
    echo "Examples:"
    echo "  $0 unified           # Run unified theory experiment"
    echo "  $0 all               # Run all experiments"
    echo "  $0 list              # List available experiments"
    echo "  $0 custom 1,2,3,4,5  # Run custom sequence"
    exit 0
fi

case $1 in
    all)
        run_all
        ;;
    list)
        echo "Available experiments:"
        for name in "${!EXPERIMENTS[@]}"; do
            echo "  $name: ${EXPERIMENTS[$name]}"
        done | sort
        ;;
    custom)
        if [ -z "$2" ]; then
            echo "Usage: $0 custom SEQUENCE"
            exit 1
        fi
        echo "═══════════════════════════════════════════════════"
        echo "  Custom Experiment"
        echo "═══════════════════════════════════════════════════"
        $SWIND detect "$2"
        ;;
    *)
        run_experiment "$1"
        ;;
esac
