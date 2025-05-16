#!/usr/bin/env -S uv run --script
# /// script
# dependencies = [
#   "seaborn"
# ]
# ///

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import seaborn as sns
from matplotlib.gridspec import GridSpec

def plot_benchmark_results(csv_file):
    # Set a consistent, professional style across all plots
    sns.set_style("darkgrid")
    plt.rcParams.update({
        'figure.facecolor': '#f0f0f0',  # Light gray figure background
        'axes.facecolor': '#e6e6e6',    # Slightly darker gray for plot area
        'font.size': 12,
        'axes.labelsize': 14,
        'axes.titlesize': 16,
        'xtick.labelsize': 12,
        'ytick.labelsize': 12,
        'legend.fontsize': 12,
        'figure.titlesize': 18,
        'grid.color': '#cccccc',        # Light gray grid lines
        'grid.linestyle': '--',
        'grid.linewidth': 0.7,
        'axes.grid': True,
        'axes.edgecolor': '#999999',    # Dark gray axes edges
        'axes.linewidth': 1.2,
        'figure.figsize': (14, 8),      # Default figure size
    })

    # Read the data
    df = pd.read_csv(csv_file)

    # Skip the first run for each mode (warm-up)
    df = df[df['run'] > 1]

    # Compute statistics for all metrics
    stats = df.groupby('mode').agg({
        'real': ['mean', 'min', 'max', 'std', 'median'],
        'user': ['mean', 'std'],
        'sys': ['mean', 'std'],
        'max_memory_kb': ['mean', 'max', 'std'],
        'cpu_percent': ['mean', 'max', 'std']
    })

    # Flatten column names
    stats.columns = ['_'.join(col).strip() for col in stats.columns.values]
    stats = stats.reset_index()

    # Calculate additional derived metrics
    stats['efficiency_score'] = stats['real_mean'] / (stats['max_memory_kb_mean'] / 1024)
    stats['cpu_efficiency'] = stats['real_mean'] / stats['cpu_percent_mean']
    stats['user_sys_ratio'] = stats['user_mean'] / stats['sys_mean']

    print("Benchmark Statistics:")
    print(stats.to_string())

    # Define colors for each mode
    colors = {'run': '#FF5733', 'build': '#3374FF', 'jit': '#33FF57'}

    # Create individual figures for each visualization

    # Style was already set in the initialization

    # Define a common function to create figures with consistent style
    def create_figure(title):
        fig = plt.figure()
        plt.suptitle(f'Kozeig Language Performance Benchmark - {title}', fontsize=18, fontweight='bold', y=0.98)
        plt.subplots_adjust(top=0.9)  # Make room for the figure title
        return fig, plt.gca()

    # 1. Execution Time - All Modes
    fig, ax = create_figure('Execution Time')
    plot_time_comparison(ax, stats, colors, show_all=True)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_time_all.png', dpi=300, bbox_inches='tight')
    print("All execution time comparison saved as benchmark_time_all.png")
    plt.close(fig)

    # 2. Zoomed Execution Time (Compiler vs JIT)
    fig, ax = create_figure('Compiled Execution Time')
    plot_time_comparison(ax, stats[stats['mode'] != 'run'], colors, show_all=False)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_time_compiled.png', dpi=300, bbox_inches='tight')
    print("Compiled execution time comparison saved as benchmark_time_compiled.png")
    plt.close(fig)

    # 3. Memory Usage
    fig, ax = create_figure('Memory Usage')
    plot_memory_usage(ax, stats, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_memory.png', dpi=300, bbox_inches='tight')
    print("Memory usage comparison saved as benchmark_memory.png")
    plt.close(fig)

    # 4. CPU Usage
    fig, ax = create_figure('CPU Usage')
    plot_cpu_usage(ax, stats, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_cpu.png', dpi=300, bbox_inches='tight')
    print("CPU usage comparison saved as benchmark_cpu.png")
    plt.close(fig)

    # 5. User/System Time Breakdown
    fig, ax = create_figure('User/System Time Breakdown')
    plot_time_breakdown(ax, stats, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_time_breakdown.png', dpi=300, bbox_inches='tight')
    print("Time breakdown saved as benchmark_time_breakdown.png")
    plt.close(fig)

    # 6. Time Distribution (Boxplot)
    fig, ax = create_figure('Execution Time Distribution')
    plot_time_distribution(ax, df, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_distribution_boxplot.png', dpi=300, bbox_inches='tight')
    print("Time distribution boxplot saved as benchmark_distribution_boxplot.png")
    plt.close(fig)

    # Create the speedup comparison figure with clear formatting
    fig, ax = create_figure('Performance Speedup')
    create_speedup_comparison(ax, stats, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('speedup_comparison.png', dpi=300, bbox_inches='tight')
    print("Speedup comparison saved as speedup_comparison.png")
    plt.close(fig)

    # Create a KDE distribution plot for easy time comparison
    fig, ax = create_figure('Time Distribution (KDE)')
    create_kde_distribution_plot(ax, df, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('benchmark_kde.png', dpi=300, bbox_inches='tight')
    print("KDE distribution plot saved as benchmark_kde.png")
    plt.close(fig)

    # Create individual resource utilization plots

    # Memory vs Execution Time
    fig, ax = create_figure('Memory vs Execution Time')
    plot_memory_vs_time(ax, df, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('memory_vs_time.png', dpi=300, bbox_inches='tight')
    print("Memory vs Time plot saved as memory_vs_time.png")
    plt.close(fig)

    # CPU vs Execution Time
    fig, ax = create_figure('CPU vs Execution Time')
    plot_cpu_vs_time(ax, df, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('cpu_vs_time.png', dpi=300, bbox_inches='tight')
    print("CPU vs Time plot saved as cpu_vs_time.png")
    plt.close(fig)

    # Memory vs CPU
    fig, ax = create_figure('Memory vs CPU Usage')
    plot_memory_vs_cpu(ax, df, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('memory_vs_cpu.png', dpi=300, bbox_inches='tight')
    print("Memory vs CPU plot saved as memory_vs_cpu.png")
    plt.close(fig)

    # Resource Efficiency
    fig, ax = create_figure('Resource Efficiency')
    plot_resource_efficiency(ax, stats, colors)
    plt.tight_layout(pad=3.0)
    plt.savefig('resource_efficiency.png', dpi=300, bbox_inches='tight')
    print("Resource efficiency comparison saved as resource_efficiency.png")
    plt.close(fig)

def plot_time_comparison(ax, stats, colors, show_all=True):
    """Plot execution time comparison."""
    title = 'Execution Time - All Modes' if show_all else 'Execution Time - Compiler vs JIT'

    # Add some padding to the bottom to make room for annotations
    y_min = 0
    y_max = max(stats['real_mean']) * 1.2

    for mode in stats['mode']:
        mode_data = stats[stats['mode'] == mode]
        bar = ax.bar(mode, mode_data['real_mean'],
                     yerr=mode_data['real_std'],
                     capsize=10,
                     color=colors[mode],
                     alpha=0.8,
                     ecolor='black',
                     linewidth=1.5)

        height = mode_data['real_mean'].values[0]
        # Add value labels above bars with more padding
        ax.text(mode, height + (y_max - y_min) * 0.03, f'{height:.4f}s',
                ha='center', va='bottom', fontweight='bold', fontsize=12)

        # Add min/max annotations with increased spacing
        ax.annotate(f"min: {mode_data['real_min'].values[0]:.4f}s",
                   (mode, 0),
                   xytext=(0, -35),
                   textcoords='offset points',
                   ha='center', fontsize=11)

        ax.annotate(f"max: {mode_data['real_max'].values[0]:.4f}s",
                   (mode, 0),
                   xytext=(0, -55),
                   textcoords='offset points',
                   ha='center', fontsize=11)

    # Set y-axis limits with padding to make room for annotations
    ax.set_ylim(y_min, y_max)

    # Add a horizontal grid for better readability
    ax.grid(axis='y', linestyle='--', alpha=0.3, which='both')

    # Customize spines
    for spine in ['top', 'right']:
        ax.spines[spine].set_visible(False)

    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Time (seconds)', fontweight='bold', labelpad=10)
    ax.set_title(f'{title} (Lower is Better)', pad=20)

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)

def plot_memory_usage(ax, stats, colors):
    """Plot memory usage comparison."""
    # Calculate y-axis limits with padding
    y_min = 0
    y_max = max(stats['max_memory_kb_mean']) * 1.2

    for mode in stats['mode']:
        mode_data = stats[stats['mode'] == mode]
        bar = ax.bar(mode, mode_data['max_memory_kb_mean'],
                     yerr=mode_data['max_memory_kb_std'],
                     capsize=10,
                     color=colors[mode],
                     alpha=0.8,
                     ecolor='black',
                     linewidth=1.5)

        height = mode_data['max_memory_kb_mean'].values[0]
        # Convert to MB if over 1000 KB for better readability
        if height > 1000:
            ax.text(mode, height + (y_max - y_min) * 0.03, f'{height/1024:.1f} MB',
                    ha='center', va='bottom', fontweight='bold', fontsize=12)
        else:
            ax.text(mode, height + (y_max - y_min) * 0.03, f'{height:.0f} KB',
                    ha='center', va='bottom', fontweight='bold', fontsize=12)

        # Add max annotation, also with MB conversion if needed
        peak = mode_data['max_memory_kb_max'].values[0]
        if peak > 1000:
            ax.annotate(f"peak: {peak/1024:.1f} MB",
                       (mode, 0),
                       xytext=(0, -35),
                       textcoords='offset points',
                       ha='center', fontsize=11)
        else:
            ax.annotate(f"peak: {peak:.0f} KB",
                       (mode, 0),
                       xytext=(0, -35),
                       textcoords='offset points',
                       ha='center', fontsize=11)

        # Add efficiency metric (memory per execution time)
        mean_time = mode_data['real_mean'].values[0]
        if mean_time > 0:  # Avoid division by zero
            efficiency = height / mean_time
            ax.annotate(f"KB/s: {efficiency:.0f}",
                       (mode, 0),
                       xytext=(0, -55),
                       textcoords='offset points',
                       ha='center', fontsize=11)

    # Set y-axis limits with padding
    ax.set_ylim(y_min, y_max)

    # Add a horizontal grid for better readability
    ax.grid(axis='y', linestyle='--', alpha=0.3, which='both')

    # Customize spines
    for spine in ['top', 'right']:
        ax.spines[spine].set_visible(False)

    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Memory (KB)', fontweight='bold', labelpad=10)
    ax.set_title('Memory Usage Comparison (Lower is Better)', pad=20)

    # Add subtle styling
    fig = ax.figure
    fig.patch.set_alpha(0.0)
    ax.patch.set_alpha(0.0)

def plot_cpu_usage(ax, stats, colors):
    """Plot CPU usage comparison."""
    # Calculate y-axis limits with padding
    y_min = 0
    y_max = max(stats['cpu_percent_mean']) * 1.5  # Extra padding for CPU since values tend to be small

    for mode in stats['mode']:
        mode_data = stats[stats['mode'] == mode]
        bar = ax.bar(mode, mode_data['cpu_percent_mean'],
                     yerr=mode_data['cpu_percent_std'],
                     capsize=10,
                     color=colors[mode],
                     alpha=0.8,
                     ecolor='black',
                     linewidth=1.5)

        height = mode_data['cpu_percent_mean'].values[0]
        # Add value labels above bars with more padding
        ax.text(mode, height + (y_max - y_min) * 0.05, f'{height:.1f}%',
                ha='center', va='bottom', fontweight='bold', fontsize=12)

        # Add peak annotation
        ax.annotate(f"peak: {mode_data['cpu_percent_max'].values[0]:.1f}%",
                   (mode, 0),
                   xytext=(0, -35),
                   textcoords='offset points',
                   ha='center', fontsize=11)

    # Set y-axis limits with padding
    ax.set_ylim(y_min, y_max)

    # Add a horizontal grid for better readability
    ax.grid(axis='y', linestyle='--', alpha=0.3, which='both')

    # Customize spines
    for spine in ['top', 'right']:
        ax.spines[spine].set_visible(False)

    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('CPU Usage (%)', fontweight='bold', labelpad=10)
    ax.set_title('CPU Usage Comparison (Lower is Better)', pad=20)

    # Add subtle styling
    fig = ax.figure
    fig.patch.set_alpha(0.0)
    ax.patch.set_alpha(0.0)

def plot_time_breakdown(ax, stats, colors):
    """Plot breakdown of time into user and system time."""
    bar_width = 0.35
    index = np.arange(len(stats['mode']))

    # Calculate the maximum height for proper y-axis scaling
    max_height = max(stats['user_mean'].max(), stats['sys_mean'].max()) * 1.3

    # User time bars
    user_bars = ax.bar(index - bar_width/2, stats['user_mean'], bar_width,
                      label='User Time',
                      color=[colors[mode] for mode in stats['mode']],
                      edgecolor='black', linewidth=1)

    # System time bars
    sys_bars = ax.bar(index + bar_width/2, stats['sys_mean'], bar_width,
                     label='System Time',
                     color=[colors[mode] for mode in stats['mode']],
                     edgecolor='black', linewidth=1,
                     hatch='//')

    # Add data labels
    for i, (user_bar, sys_bar) in enumerate(zip(user_bars, sys_bars)):
        user_height = user_bar.get_height()
        sys_height = sys_bar.get_height()

        # Add value labels inside or above the bars depending on height
        text_y_pos = user_height + max_height * 0.02
        ax.text(user_bar.get_x() + user_bar.get_width()/2, text_y_pos,
                f'{user_height:.3f}s', ha='center', va='bottom',
                fontsize=10, fontweight='bold')

        text_y_pos = sys_height + max_height * 0.02
        ax.text(sys_bar.get_x() + sys_bar.get_width()/2, text_y_pos,
                f'{sys_height:.3f}s', ha='center', va='bottom',
                fontsize=10, fontweight='bold')

        # Calculate and show the ratio if system time is non-zero
        if sys_height > 0:
            ratio = user_height / sys_height
            if ratio > 1000:
                ratio_text = "very high"
            else:
                ratio_text = f"{ratio:.1f}x"

            ax.text(index[i], max(user_height, sys_height) * 1.15,
                    f'User/Sys: {ratio_text}', ha='center',
                    fontsize=10, color='black',
                    bbox=dict(boxstyle="round,pad=0.2", fc="white", ec="gray", alpha=0.8))

    # Set y-axis limits with padding for the labels
    ax.set_ylim(0, max_height)

    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Time (seconds)', fontweight='bold', labelpad=10)
    ax.set_title('User vs System Time Breakdown (Lower is Better)', pad=20)
    ax.set_xticks(index)
    ax.set_xticklabels(stats['mode'], fontsize=12)

    # Add a legend with better formatting
    ax.legend(frameon=True, fancybox=True, framealpha=0.8, fontsize=10,
             facecolor='white', edgecolor='gray')

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)

def plot_time_distribution(ax, df, colors):
    """Plot execution time distribution as boxplots."""
    boxplot_data = []
    labels = []

    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        boxplot_data.append(mode_data['real'])
        labels.append(mode)

    # Create box plot with enhanced styling
    bp = ax.boxplot(boxplot_data, tick_labels=labels, patch_artist=True,
                   widths=0.6, showmeans=True, meanline=True, notch=True,
                   flierprops={'marker': 'o', 'markerfacecolor': 'white', 'markeredgecolor': 'black', 'markersize': 6})

    # Enhance the appearance of the box plot
    for element in ['boxes', 'whiskers', 'fliers', 'means', 'medians', 'caps']:
        for item in bp[element]:
            if element == 'boxes':
                # Make the boxes semi-transparent
                item.set(linewidth=2)
            else:
                item.set(linewidth=1.5)

    # Color the boxplots
    for i, box in enumerate(bp['boxes']):
        box.set(facecolor=colors[labels[i]], alpha=0.6, edgecolor='black')

    # Style the means and medians
    for mean in bp['means']:
        mean.set(linestyle='-', linewidth=2, color='black')
    for median in bp['medians']:
        median.set(color='black', linewidth=2)

    # Add individual data points (jittered)
    for i, mode in enumerate(df['mode'].unique()):
        mode_data = df[df['mode'] == mode]
        x = np.random.normal(i+1, 0.05, size=len(mode_data))
        ax.scatter(x, mode_data['real'], s=40, alpha=0.5, color=colors[mode], edgecolor='white', linewidth=0.5)

    # Add annotations with key statistics
    for i, mode in enumerate(df['mode'].unique()):
        mode_data = df[df['mode'] == mode]
        # Add min, mean, median, max and standard deviation
        stats_text = f"mean: {mode_data['real'].mean():.4f}s\nmedian: {mode_data['real'].median():.4f}s"
        ax.annotate(stats_text, xy=(i+1, mode_data['real'].min()),
                   xytext=(0, -60), textcoords='offset points',
                   ha='center', va='top', fontsize=10, color='black',
                   bbox=dict(boxstyle="round,pad=0.3", fc="white", ec="gray", alpha=0.8))

    # Customize spines
    for spine in ['top', 'right']:
        ax.spines[spine].set_visible(False)

    # Add a horizontal grid for better readability
    ax.grid(axis='y', linestyle='--', alpha=0.3, which='both')

    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Time (seconds)', fontweight='bold', labelpad=10)
    ax.set_title('Execution Time Distribution (Lower is Better)', pad=20)

    # Add subtle styling
    fig = ax.figure
    fig.patch.set_alpha(0.0)
    ax.patch.set_alpha(0.0)

def create_speedup_comparison(ax, stats, colors):
    """Create a separate figure for speedup comparison."""
    interpreter_time = stats[stats['mode'] == 'run']['real_mean'].values[0]

    # Create cleaner bars with consistent formatting
    modes = []
    speedups = []
    bar_colors = []

    # Add the interpreter as baseline 1.0x
    modes.append('run')
    speedups.append(1.0)
    bar_colors.append(colors['run'])

    # Add other modes
    for mode in stats['mode']:
        if mode != 'run':
            mode_time = stats[stats['mode'] == mode]['real_mean'].values[0]
            speedup = interpreter_time / mode_time
            modes.append(mode)
            speedups.append(speedup)
            bar_colors.append(colors[mode])

    # Calculate standard deviation for error bars
    # This should give a better estimate of the confidence in speedup values
    error_bars = []
    for mode, speedup in zip(modes, speedups):
        if mode == 'run':
            error_bars.append(0)  # No error for baseline
        else:
            # Calculate error propagation
            run_mean = stats[stats['mode'] == 'run']['real_mean'].values[0]
            run_std = stats[stats['mode'] == 'run']['real_std'].values[0]
            mode_mean = stats[stats['mode'] == mode]['real_mean'].values[0]
            mode_std = stats[stats['mode'] == mode]['real_std'].values[0]

            # Error propagation formula for division (f = a/b): σf ≈ f * sqrt((σa/a)² + (σb/b)²)
            relative_error = speedup * np.sqrt((run_std/run_mean)**2 + (mode_std/mode_mean)**2)
            error_bars.append(relative_error)

    # Create the plot with error bars
    bars = ax.bar(modes, speedups, color=bar_colors,
             yerr=error_bars, capsize=10, ecolor='black', linewidth=1.5)

    # Add text annotations
    for i, (mode, speedup) in enumerate(zip(modes, speedups)):
        ax.text(i, speedup + max(speedups) * 0.05, f'{speedup:.1f}x',
                ha='center', va='bottom', fontweight='bold', fontsize=14)

    # Set y-axis limits with padding
    y_max = max(speedups) * 1.2
    ax.set_ylim(0, y_max)

    # Add baseline reference line
    ax.axhline(y=1.0, color='gray', linestyle='--', alpha=0.5)

    # Set labels and title
    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Speedup Factor', fontweight='bold', labelpad=10)
    ax.set_title('Performance Speedup Relative to Interpreter (Higher is Better)', pad=20)

def create_time_distribution_plot(df, colors):
    """Create a detailed time distribution plot."""
    # Create KDE plots for each mode
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        sns.kdeplot(mode_data['real'], fill=True, alpha=0.3,
                   label=f"{mode} (mean: {mode_data['real'].mean():.4f}s)",
                   color=colors[mode])

    # Add actual data points at the bottom of the plot
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        y_pos = -0.01  # Position below the x-axis
        plt.scatter(mode_data['real'],
                   [y_pos] * len(mode_data),
                   color=colors[mode],
                   alpha=0.7,
                   s=20)

    plt.title('Execution Time Distribution Across Modes')
    plt.xlabel('Time (seconds)')
    plt.ylabel('Density')
    plt.legend()
    plt.grid(True, alpha=0.3)

def create_kde_distribution_plot(ax, df, colors):
    """Create a simple KDE distribution plot for execution time."""
    # Determine if we need separate plots for compiled modes
    has_large_difference = False
    if 'run' in df['mode'].unique() and 'build' in df['mode'].unique():
        run_mean = df[df['mode'] == 'run']['real'].mean()
        build_mean = df[df['mode'] == 'build']['real'].mean()
        if run_mean / build_mean > 10:  # If the difference is more than 10x
            has_large_difference = True

    # Plot each execution mode with a KDE (Kernel Density Estimation)
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        sns.kdeplot(mode_data['real'], fill=True, alpha=0.4,
                   label=f"{mode} (mean: {mode_data['real'].mean():.4f}s)",
                   color=colors[mode], linewidth=2, ax=ax)

    # Add a rug plot (small ticks) showing the actual data points
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        ax.plot(mode_data['real'], [0] * len(mode_data), '|',
               color=colors[mode], alpha=0.7, markersize=20)

        # Add vertical lines for means
        ax.axvline(x=mode_data['real'].mean(), color=colors[mode],
                  linestyle='--', linewidth=1.5, alpha=0.7)

    # Add annotations for means
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        mean_val = mode_data['real'].mean()
        # Calculate an appropriate y-position for the label
        if has_large_difference and mode == 'run':
            y_pos = 0.8  # Place it high up for clarity when run is much slower
        else:
            # Create a temporary KDE plot to extract the density values
            temp_fig, temp_ax = plt.subplots()
            temp_kde = sns.kdeplot(mode_data['real'], ax=temp_ax)
            kde_data = temp_kde.get_lines()[-1].get_data()
            plt.close(temp_fig)  # Close the temporary figure

            # Find the closest x value to our mean
            y_pos_idx = np.abs(kde_data[0] - mean_val).argmin()
            y_pos = kde_data[1][y_pos_idx] * 1.1

        ax.annotate(f"{mode}: {mean_val:.4f}s",
                   xy=(mean_val, y_pos),
                   xytext=(10, 0), textcoords='offset points',
                   fontsize=11, color=colors[mode],
                   bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=colors[mode], alpha=0.8))

    ax.set_xlabel('Time (seconds)', fontweight='bold', labelpad=10)
    ax.set_ylabel('Density', fontweight='bold', labelpad=10)
    ax.set_title('Execution Time Distribution (Lower is Better)', pad=20)

    # If we have large differences, add an inset for the faster execution modes
    if has_large_difference:
        try:
            from mpl_toolkits.axes_grid1.inset_locator import inset_axes

            # Create an inset for the compiled modes
            axins = inset_axes(ax, width="40%", height="30%", loc='upper right')

            # Only plot the compiled modes in the inset
            compiled_modes = [mode for mode in df['mode'].unique() if mode != 'run']
            for mode in compiled_modes:
                mode_data = df[df['mode'] == mode]
                sns.kdeplot(mode_data['real'], fill=True, alpha=0.4,
                           label=mode, color=colors[mode], ax=axins)
                # Add rug plot
                axins.plot(mode_data['real'], [0] * len(mode_data), '|',
                         color=colors[mode], alpha=0.7, markersize=10)

            axins.set_title('Compiled Modes (Zoomed)', fontsize=10)
            axins.tick_params(labelsize=8)
            axins.grid(True, alpha=0.3)

            # Add a legend to the inset
            axins.legend(fontsize=8)
        except Exception as e:
            print(f"Could not create inset: {e}")

    # Add a legend with better formatting
    ax.legend(frameon=True, fancybox=True, framealpha=0.8, fontsize=10,
             facecolor='white', edgecolor='gray')

def plot_memory_vs_time(ax, df, colors):
    """Plot memory usage vs execution time."""
    # Add a scatter plot point for each run, with larger markers
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]
        scatter = ax.scatter(mode_data['real'], mode_data['max_memory_kb'],
                   label=mode, color=colors[mode], s=80,
                   edgecolors='white', linewidth=0.5)

        # Add ellipses to show data distribution
        if len(mode_data) > 2:  # Need at least 3 points for covariance
            from matplotlib.patches import Ellipse

            # Calculate mean and covariance
            mean_x = mode_data['real'].mean()
            mean_y = mode_data['max_memory_kb'].mean()

            # Skip ellipses if the data doesn't have enough variation
            if mode_data['real'].std() > 0 and mode_data['max_memory_kb'].std() > 0:
                try:
                    cov = np.cov(mode_data['real'], mode_data['max_memory_kb'])

                    # Calculate eigenvalues and eigenvectors for the ellipse
                    if not np.isnan(cov).any() and not np.isinf(cov).any() and cov.size > 0:
                        eigenvals, eigenvecs = np.linalg.eigh(cov)
                        # Using 2 standard deviations for 95% confidence
                        width, height = 2 * np.sqrt(eigenvals)
                        theta = np.degrees(np.arctan2(eigenvecs[1, 0], eigenvecs[0, 0]))

                        # Create ellipse
                        ellipse = Ellipse(xy=(mean_x, mean_y),
                                        width=width, height=height,
                                        angle=theta, edgecolor=colors[mode],
                                        fc=colors[mode], lw=2, alpha=0.2)
                        ax.add_patch(ellipse)

                        # Mark the center of each cluster
                        ax.scatter([mean_x], [mean_y], marker='x', color=colors[mode],
                                 s=100, linewidth=2, label=f'{mode} (mean)', alpha=0.8)
                except Exception as e:
                    print(f"Skipping ellipse for {mode}: {e}")

    ax.set_xlabel('Execution Time (s)', fontweight='bold', labelpad=10)
    ax.set_ylabel('Memory Usage (KB)', fontweight='bold', labelpad=10)
    ax.set_title('Memory Usage vs. Execution Time (Lower is Better for Both)', pad=20)

    # Add a legend with better formatting
    handles, labels = ax.get_legend_handles_labels()
    # Filter out duplicates that might have been added by the mean markers
    unique_labels = []
    unique_handles = []
    for handle, label in zip(handles, labels):
        if "(mean)" not in label or label not in unique_labels:
            unique_labels.append(label)
            unique_handles.append(handle)

    ax.legend(unique_handles, unique_labels, frameon=True, fancybox=True,
             framealpha=0.8, fontsize=10, facecolor='white', edgecolor='gray')

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)

def plot_cpu_vs_time(ax, df, colors):
    """Plot CPU usage vs execution time."""
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]

        # Add scatter plot with larger markers and white edge
        ax.scatter(mode_data['real'], mode_data['cpu_percent'],
                  label=mode, color=colors[mode], s=80,
                  edgecolors='white', linewidth=0.5)

        # Calculate the mean values
        mean_x = mode_data['real'].mean()
        mean_y = mode_data['cpu_percent'].mean()

        # Add a point for the mean with an X marker
        ax.scatter([mean_x], [mean_y], marker='x', color=colors[mode],
                  s=100, linewidth=2)

        # Add text annotation for mean values
        ax.annotate(f"{mode} mean: ({mean_x:.3f}s, {mean_y:.1f}%)",
                   xy=(mean_x, mean_y), xytext=(10, 5),
                   textcoords="offset points", fontsize=10,
                   bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=colors[mode], alpha=0.8))

    ax.set_xlabel('Execution Time (s)', fontweight='bold', labelpad=10)
    ax.set_ylabel('CPU Usage (%)', fontweight='bold', labelpad=10)
    ax.set_title('CPU Usage vs. Execution Time (Lower is Better for Both)', pad=20)

    # Add a legend with better formatting
    ax.legend(frameon=True, fancybox=True, framealpha=0.8, fontsize=10,
             facecolor='white', edgecolor='gray')

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)

def plot_memory_vs_cpu(ax, df, colors):
    """Plot memory usage vs CPU usage."""
    for mode in df['mode'].unique():
        mode_data = df[df['mode'] == mode]

        # Add scatter plot with larger markers and white edge
        ax.scatter(mode_data['cpu_percent'], mode_data['max_memory_kb'],
                  label=mode, color=colors[mode], s=80,
                  edgecolors='white', linewidth=0.5)

        # Calculate the mean values
        mean_x = mode_data['cpu_percent'].mean()
        mean_y = mode_data['max_memory_kb'].mean()

        # Add a point for the mean with an X marker
        ax.scatter([mean_x], [mean_y], marker='x', color=colors[mode],
                  s=100, linewidth=2)

        # Add text annotation for mean values
        # Convert to MB if over 1000 KB for better readability
        if mean_y > 1000:
            mem_str = f"{mean_y/1024:.1f} MB"
        else:
            mem_str = f"{mean_y:.0f} KB"

        ax.annotate(f"{mode} mean: ({mean_x:.1f}%, {mem_str})",
                   xy=(mean_x, mean_y), xytext=(10, 5),
                   textcoords="offset points", fontsize=10,
                   bbox=dict(boxstyle="round,pad=0.3", fc="white", ec=colors[mode], alpha=0.8))

    ax.set_xlabel('CPU Usage (%)', fontweight='bold', labelpad=10)
    ax.set_ylabel('Memory Usage (KB)', fontweight='bold', labelpad=10)
    ax.set_title('Memory Usage vs. CPU Usage (Lower is Better for Both)', pad=20)

    # Add a legend with better formatting
    ax.legend(frameon=True, fancybox=True, framealpha=0.8, fontsize=10,
             facecolor='white', edgecolor='gray')

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)

def plot_resource_efficiency(ax, stats, colors):
    """Plot resource efficiency comparison."""
    # Create a bar chart for performance comparison
    modes = stats['mode']
    x = np.arange(len(modes))
    width = 0.25  # Slightly wider bars

    # Normalize the metrics for comparison on the same scale
    time_norm = stats['real_mean'] / stats['real_mean'].max()
    mem_norm = stats['max_memory_kb_mean'] / stats['max_memory_kb_mean'].max()
    cpu_norm = stats['cpu_percent_mean'] / stats['cpu_percent_mean'].max() if stats['cpu_percent_mean'].max() > 0 else stats['cpu_percent_mean']

    # Inverse time (higher is better, like the other metrics)
    time_eff = 1 - time_norm
    mem_eff = 1 - mem_norm
    cpu_eff = 1 - cpu_norm

    # Create bar plots with explicit colors and hatching for better distinction
    time_bars = ax.bar(x - width, time_eff, width, label='Time Efficiency',
                      color=[colors[mode] for mode in modes],
                      edgecolor='black', linewidth=1)

    mem_bars = ax.bar(x, mem_eff, width, label='Memory Efficiency',
                     color=[colors[mode] for mode in modes],
                     edgecolor='black', linewidth=1,
                     hatch='//')

    cpu_bars = ax.bar(x + width, cpu_eff, width, label='CPU Efficiency',
                     color=[colors[mode] for mode in modes],
                     edgecolor='black', linewidth=1,
                     hatch='\\\\')

    # Add value labels to each bar
    def add_labels(bars, values):
        for bar, value in zip(bars, values):
            height = bar.get_height()
            ax.text(bar.get_x() + bar.get_width()/2., height + 0.02,
                   f"{value:.2f}", ha='center', va='bottom', fontsize=10)

    add_labels(time_bars, time_eff)
    add_labels(mem_bars, mem_eff)
    add_labels(cpu_bars, cpu_eff)

    # Add a horizontal line at 0.5 for reference
    ax.axhline(y=0.5, color='gray', linestyle='--', alpha=0.5)

    # Customize the axis
    ax.set_ylim(0, 1.1)  # Set y-axis from 0 to 1.1 to make room for labels
    ax.set_xlabel('Execution Mode', fontweight='bold', labelpad=10)
    ax.set_ylabel('Efficiency Score (Higher is Better)', fontweight='bold', labelpad=10)
    ax.set_title('Resource Efficiency Comparison (Higher is Better)', pad=20)
    ax.set_xticks(x)
    ax.set_xticklabels(modes, fontsize=12)

    # Add a legend with better formatting
    ax.legend(frameon=True, fancybox=True, framealpha=0.8, fontsize=10,
             facecolor='white', edgecolor='gray', loc='upper right')

    # Use consistent grid styling
    ax.grid(True, which='major', linestyle='--', linewidth=0.7, alpha=0.7)


if __name__ == "__main__":
    if len(sys.argv) > 1:
        csv_file = sys.argv[1]
    else:
        csv_file = "benchmark_results.csv"

    plot_benchmark_results(csv_file)
