#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys

def plot_benchmark_results(csv_file):
    df = pd.read_csv(csv_file)

    df = df[df['run'] > 1]

    stats = df.groupby('mode').agg({
        'real': ['mean', 'min', 'max', 'std'],
        'user': ['mean'],
        'sys': ['mean']
    })

    stats.columns = ['_'.join(col).strip() for col in stats.columns.values]
    stats = stats.reset_index()

    print("Benchmark Statistics:")
    print(stats)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))

    colors = {'run': 'red', 'build': 'blue', 'jit': 'green'}

    for mode in stats['mode']:
        mode_data = stats[stats['mode'] == mode]
        bar = ax1.bar(mode, mode_data['real_mean'],
                      yerr=mode_data['real_std'],
                      capsize=10,
                      color=colors[mode],
                      alpha=0.7)

        height = mode_data['real_mean'].values[0]
        ax1.text(mode, height + 0.01, f'{height:.4f}s',
                ha='center', va='bottom', rotation=0)

        y_offset = -0.03 if mode == 'run' else -0.01
        ax1.annotate(f"min: {mode_data['real_min'].values[0]:.4f}s",
                    (mode, 0),
                    xytext=(0, y_offset),
                    textcoords='axes fraction',
                    ha='center')

        y_offset = -0.06 if mode == 'run' else -0.02
        ax1.annotate(f"max: {mode_data['real_max'].values[0]:.4f}s",
                    (mode, 0),
                    xytext=(0, y_offset),
                    textcoords='axes fraction',
                    ha='center')

    ax1.set_xlabel('Execution Mode')
    ax1.set_ylabel('Time (seconds)')
    ax1.set_title('Lut Language Performance Comparison - All Modes')
    ax1.grid(axis='y', linestyle='--', alpha=0.7)

    filtered_stats = stats[stats['mode'] != 'run']

    for mode in filtered_stats['mode']:
        mode_data = filtered_stats[filtered_stats['mode'] == mode]
        bar = ax2.bar(mode, mode_data['real_mean'],
                      yerr=mode_data['real_std'],
                      capsize=10,
                      color=colors[mode],
                      alpha=0.7)

        height = mode_data['real_mean'].values[0]
        ax2.text(mode, height + 0.0005, f'{height:.6f}s',
                ha='center', va='bottom', rotation=0)

        ax2.annotate(f"min: {mode_data['real_min'].values[0]:.4f}s",
                    (mode, 0),
                    xytext=(0, -25),
                    textcoords='offset points',
                    ha='center')

        ax2.annotate(f"max: {mode_data['real_max'].values[0]:.4f}s",
                    (mode, 0),
                    xytext=(0, -40),
                    textcoords='offset points',
                    ha='center')

    ax2.set_xlabel('Execution Mode')
    ax2.set_ylabel('Time (seconds)')
    ax2.set_title('Zoomed View: Compiler vs JIT Performance')
    ax2.grid(axis='y', linestyle='--', alpha=0.7)

    plt.tight_layout()

    plt.savefig('benchmark_comparison.png', dpi=300)
    print("Enhanced plot saved as benchmark_comparisonpng")

    plt.figure(figsize=(10, 6))

    interpreter_time = stats[stats['mode'] == 'run']['real_mean'].values[0]
    for mode in stats['mode']:
        if mode != 'run':
            mode_time = stats[stats['mode'] == mode]['real_mean'].values[0]
            speedup = interpreter_time / mode_time

            plt.bar(mode, speedup, color=colors[mode], alpha=0.7)
            plt.text(mode, speedup + 5, f'{speedup:.1f}x faster',
                    ha='center', va='bottom', fontweight='bold')

    plt.title('Performance Speedup Relative to Interpreter')
    plt.xlabel('Execution Mode')
    plt.ylabel('Speedup Factor (higher is better)')
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    plt.tight_layout()
    plt.savefig('speedup_comparison.png', dpi=300)
    print("Speedup comparison plot saved as speedup_comparison.png")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        csv_file = sys.argv[1]
    else:
        csv_file = "benchmark_results.csv"

    plot_benchmark_results(csv_file)
