#!/bin/bash

# Benchmark script for Lut language performance comparison
# This script runs the factorial.lut example 50 times in each mode
# (interpreter, compiler, JIT) and records execution times to a CSV file

echo "Building Lut in release mode..."
cargo build --release

if [ -f benchmark_results.csv ]; then
    rm benchmark_results.csv
fi

echo "mode,run,real,user,sys" > benchmark_results.csv

run_benchmark() {
    local mode=$1
    local run=$2

    result=$(/usr/bin/time -p ./target/release/lut $mode ./examples/factorial.lut 2>&1 | grep -E "^(real|user|sys)" | awk '{print $2}')

    real=$(echo "$result" | head -n 1)
    user=$(echo "$result" | head -n 2 | tail -n 1)
    sys=$(echo "$result" | tail -n 1)

    echo "$mode,$run,$real,$user,$sys" >> benchmark_results.csv

    echo "Completed $mode run $run: real=$real, user=$user, sys=$sys"
}

calculate_stats() {
    local mode=$1
    echo "===== Statistics for $mode mode ====="

    avg_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $3; count++ } END { print sum/count }' benchmark_results.csv)

    min_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (min == "" || $3 < min) min = $3 } END { print min }' benchmark_results.csv)
    max_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (max == "" || $3 > max) max = $3 } END { print max }' benchmark_results.csv)

    avg_user=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $4; count++ } END { print sum/count }' benchmark_results.csv)

    avg_sys=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $5; count++ } END { print sum/count }' benchmark_results.csv)

    echo "Average real time: $avg_real seconds"
    echo "Min real time: $min_real seconds"
    echo "Max real time: $max_real seconds"
    echo "Average user time: $avg_user seconds"
    echo "Average sys time: $avg_sys seconds"
    echo ""
}

echo "Running interpreter mode benchmarks..."
for i in {1..50}; do
    run_benchmark "run" $i
done

echo "Running compiler mode benchmarks..."
for i in {1..50}; do
    run_benchmark "build" $i
done

echo "Running JIT mode benchmarks..."
for i in {1..50}; do
    run_benchmark "jit" $i
done

echo "Benchmark complete. Results saved to benchmark_results.csv"

calculate_stats "run"
calculate_stats "build"
calculate_stats "jit"

echo "mode,avg_real,avg_user,avg_sys" > benchmark_summary.csv
for mode in "run" "build" "jit"; do
    avg_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $3; count++ } END { print sum/count }' benchmark_results.csv)
    avg_user=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $4; count++ } END { print sum/count }' benchmark_results.csv)
    avg_sys=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $5; count++ } END { print sum/count }' benchmark_results.csv)
    echo "$mode,$avg_real,$avg_user,$avg_sys" >> benchmark_summary.csv
done

echo "Summary statistics saved to benchmark_summary.csv"
