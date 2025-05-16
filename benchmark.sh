#!/bin/bash

# Enhanced benchmark script for Kozeig language performance comparison
# This script runs the benchmark.ko example with detailed performance metrics

echo "Building Kozeig in release mode..."
cargo build --release

if [ -f benchmark_results.csv ]; then
    rm benchmark_results.csv
fi

# Added CPU and memory metrics to CSV header
echo "mode,run,real,user,sys,max_memory_kb,cpu_percent" > benchmark_results.csv

# Check if GNU time is available (needed for memory measurements)
if command -v /usr/bin/time >/dev/null 2>&1; then
    TIME_CMD="/usr/bin/time"
elif command -v gtime >/dev/null 2>&1; then
    TIME_CMD="gtime"
else
    echo "WARNING: Neither GNU time nor gtime found. Memory usage will not be measured."
    TIME_CMD="time"
fi

run_benchmark() {
    local mode=$1
    local run=$2

    # Create a temporary file for collecting detailed metrics
    local temp_file=$(mktemp)

    # Check the OS type
    if [[ "$(uname)" == "Darwin" ]]; then
        # macOS approach - use time's built-in output format with mach_absolute_time for more precision
        # and ps with a PID file to monitor the actual process

        # Create a PID file to track the Lut process
        pid_file=$(mktemp)

        # Start timing - macOS doesn't support %N in date, so we use perl for high precision
        start_time=$(perl -MTime::HiRes=time -e 'printf "%.6f", time')

        # Run the command and save PID
        (/usr/bin/time -p ./target/release/koze $mode ./examples/benchmark.ko > /dev/null 2> $temp_file & echo $! > $pid_file)
        lut_pid=$(cat $pid_file)

        # Monitor memory usage every 0.1 seconds
        max_memory=0
        max_cpu=0

        # Keep monitoring until process completes
        while kill -0 $lut_pid 2>/dev/null; do
            # Get memory and CPU for the actual Lut process
            if ps -p $lut_pid >/dev/null 2>&1; then
                current_mem=$(ps -o rss= -p $lut_pid | tr -d ' ')
                current_cpu=$(ps -o %cpu= -p $lut_pid | tr -d ' ')

                # Update max values if higher
                if [[ -n "$current_mem" && $current_mem -gt $max_memory ]]; then
                    max_memory=$current_mem
                fi

                if [[ -n "$current_cpu" && $(echo "$current_cpu > $max_cpu" | bc -l) -eq 1 ]]; then
                    max_cpu=$current_cpu
                fi
            fi
            sleep 0.1
        done

        # Use the max values we collected
        memory=$max_memory
        cpu=$max_cpu

        # Calculate elapsed time with high precision
        end_time=$(perl -MTime::HiRes=time -e 'printf "%.6f", time')
        real=$(echo "$end_time - $start_time" | bc)

        # Extract user and sys time from time command output
        user=$(grep "user" $temp_file | awk '{print $2}')
        sys=$(grep "sys" $temp_file | awk '{print $2}')

        # Clean up the PID file
        rm -f $pid_file
    else
        # Linux with GNU time approach
        $TIME_CMD -f "%e,%U,%S,%M,%P" ./target/release/koze $mode ./examples/benchmark.ko > /dev/null 2> $temp_file

        # Parse the metrics from the temp file
        metrics=$(cat $temp_file)

        # Extract values
        real=$(echo "$metrics" | cut -d',' -f1)
        user=$(echo "$metrics" | cut -d',' -f2)
        sys=$(echo "$metrics" | cut -d',' -f3)
        memory=$(echo "$metrics" | cut -d',' -f4)
        cpu=$(echo "$metrics" | cut -d',' -f5 | sed 's/%//')
    fi

    # Cleanup
    rm $temp_file

    # Append to CSV
    echo "$mode,$run,$real,$user,$sys,$memory,$cpu" >> benchmark_results.csv

    echo "Completed $mode run $run: real=${real}s, user=${user}s, sys=${sys}s, mem=${memory}KB, CPU=${cpu}%"
}

calculate_stats() {
    local mode=$1
    echo "===== Statistics for $mode mode ====="

    avg_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $3; count++ } END { print sum/count }' benchmark_results.csv)
    min_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (min == "" || $3 < min) min = $3 } END { print min }' benchmark_results.csv)
    max_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (max == "" || $3 > max) max = $3 } END { print max }' benchmark_results.csv)

    avg_user=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $4; count++ } END { print sum/count }' benchmark_results.csv)
    avg_sys=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $5; count++ } END { print sum/count }' benchmark_results.csv)

    avg_mem=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $6; count++ } END { print sum/count }' benchmark_results.csv)
    max_mem=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (max == "" || $6 > max) max = $6 } END { print max }' benchmark_results.csv)

    avg_cpu=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $7; count++ } END { print sum/count }' benchmark_results.csv)

    echo "Average real time: $avg_real seconds"
    echo "Min real time: $min_real seconds"
    echo "Max real time: $max_real seconds"
    echo "Average user time: $avg_user seconds"
    echo "Average sys time: $avg_sys seconds"
    echo "Average memory usage: $avg_mem KB"
    echo "Peak memory usage: $max_mem KB"
    echo "Average CPU usage: $avg_cpu%"
    echo ""
}

# Reduced number of runs for testing purposes
# Increase this for more statistically significant results
RUNS=50

echo "Running interpreter mode benchmarks..."
for i in $(seq 1 $RUNS); do
    run_benchmark "run" $i
done

echo "Running compiler mode benchmarks..."
for i in $(seq 1 $RUNS); do
    run_benchmark "build" $i
done

echo "Running JIT mode benchmarks..."
for i in $(seq 1 $RUNS); do
    run_benchmark "jit" $i
done

echo "Benchmark complete. Results saved to benchmark_results.csv"

calculate_stats "run"
calculate_stats "build"
calculate_stats "jit"

# Create a more detailed summary CSV with the new metrics
echo "mode,avg_real,min_real,max_real,avg_user,avg_sys,avg_memory_kb,max_memory_kb,avg_cpu_percent" > benchmark_summary.csv

for mode in "run" "build" "jit"; do
    avg_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $3; count++ } END { print sum/count }' benchmark_results.csv)
    min_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (min == "" || $3 < min) min = $3 } END { print min }' benchmark_results.csv)
    max_real=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (max == "" || $3 > max) max = $3 } END { print max }' benchmark_results.csv)

    avg_user=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $4; count++ } END { print sum/count }' benchmark_results.csv)
    avg_sys=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $5; count++ } END { print sum/count }' benchmark_results.csv)

    avg_mem=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $6; count++ } END { print sum/count }' benchmark_results.csv)
    max_mem=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { if (max == "" || $6 > max) max = $6 } END { print max }' benchmark_results.csv)

    avg_cpu=$(awk -F, -v mode="$mode" '$1 == mode && $2 > 1 { sum += $7; count++ } END { print sum/count }' benchmark_results.csv)

    echo "$mode,$avg_real,$min_real,$max_real,$avg_user,$avg_sys,$avg_mem,$max_mem,$avg_cpu" >> benchmark_summary.csv
done

echo "Enhanced summary statistics saved to benchmark_summary.csv"

# Run the Rust visualization if the binary exists
if [ -f "./target/release/benchmark" ]; then
    echo "Generating visualization using Rust benchmark tool..."
    ./target/release/benchmark
else
    echo "Building Rust benchmark tool..."
    cargo build --release --bin benchmark
    if [ -f "./target/release/benchmark" ]; then
        echo "Generating visualization using Rust benchmark tool..."
        ./target/release/benchmark
    fi
fi
