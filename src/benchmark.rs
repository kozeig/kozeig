use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use csv::Reader;
use plotters::prelude::*;

type PlotResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct BenchmarkRun {
    mode: String,
    run: i32,
    real: f64,
    user: f64,
    sys: f64,
    max_memory_kb: f64,
    cpu_percent: f64,
}

struct Statistics {
    mean: f64,
    std: f64,
    min: f64,
    max: f64,
    median: f64,
}

fn calculate_stats(values: &[f64]) -> Statistics {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    
    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / n;
    let std = variance.sqrt();
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };
    
    Statistics {
        mean,
        std,
        min: *sorted.first().unwrap(),
        max: *sorted.last().unwrap(),
        median,
    }
}

fn read_benchmark_data(path: &Path) -> PlotResult<Vec<BenchmarkRun>> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut runs = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        runs.push(BenchmarkRun {
            mode: record[0].to_string(),
            run: record[1].parse()?,
            real: record[2].parse()?,
            user: record[3].parse()?,
            sys: record[4].parse()?,
            max_memory_kb: record[5].parse()?,
            cpu_percent: record[6].parse()?,
        });
    }
    
    // Skip warm-up runs (run > 1)
    runs.retain(|r| r.run > 1);
    
    Ok(runs)
}

fn plot_execution_time(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?; // Grey background
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.real);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes_data.values()
        .flat_map(|v| v.iter())
        .fold(0f64, |a, &b| f64::max(a, b)) * 1.2;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - Execution Time", ("Arial", 36).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Time (seconds)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    for (i, mode) in modes.iter().enumerate() {
        if let Some(values) = modes_data.get(*mode) {
            let stats = calculate_stats(values);
            
            chart.draw_series(std::iter::once(Rectangle::new([
                (i as f64 + 0.3, 0.0),
                (i as f64 + 0.7, stats.mean)
            ], colors[i].filled())))?;
            
            // Error bars
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (i as f64 + 0.5, stats.mean - stats.std),
                (i as f64 + 0.5, stats.mean + stats.std)
            ], &BLACK)))?;
            
            // Add labels
            chart.draw_series(std::iter::once(Text::new(
                format!("{:.4}s", stats.mean),
                (i as f64 + 0.5, stats.mean + y_max * 0.05),
                ("Arial", 20).into_font(),
            )))?;
            
            // Min/Max annotations
            chart.draw_series(std::iter::once(Text::new(
                format!("min: {:.4}s", stats.min),
                (i as f64 + 0.5, -y_max * 0.08),
                ("Arial", 16).into_font(),
            )))?;
            
            chart.draw_series(std::iter::once(Text::new(
                format!("max: {:.4}s", stats.max),
                (i as f64 + 0.5, -y_max * 0.14),
                ("Arial", 16).into_font(),
            )))?;
        }
    }
    
    root.present()?;
    println!("Execution time comparison saved as {}", output_path);
    Ok(())
}

fn plot_memory_usage(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.max_memory_kb);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes_data.values()
        .flat_map(|v| v.iter())
        .fold(0f64, |a, &b| f64::max(a, b)) * 1.2;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - Memory Usage", ("Arial", 36).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Memory (KB)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    for (i, mode) in modes.iter().enumerate() {
        if let Some(values) = modes_data.get(*mode) {
            let stats = calculate_stats(values);
            
            chart.draw_series(std::iter::once(Rectangle::new([
                (i as f64 + 0.3, 0.0),
                (i as f64 + 0.7, stats.mean)
            ], colors[i].filled())))?;
            
            // Error bars
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (i as f64 + 0.5, stats.mean - stats.std),
                (i as f64 + 0.5, stats.mean + stats.std)
            ], &BLACK)))?;
            
            // Add labels
            let label = if stats.mean > 1000.0 {
                format!("{:.1} MB", stats.mean / 1024.0)
            } else {
                format!("{:.0} KB", stats.mean)
            };
            
            chart.draw_series(std::iter::once(Text::new(
                label,
                (i as f64 + 0.5, stats.mean + y_max * 0.05),
                ("Arial", 20).into_font(),
            )))?;
        }
    }
    
    root.present()?;
    println!("Memory usage comparison saved as {}", output_path);
    Ok(())
}

fn plot_cpu_usage(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.cpu_percent);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes_data.values()
        .flat_map(|v| v.iter())
        .fold(0f64, |a, &b| f64::max(a, b)) * 1.5;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - CPU Usage", ("Arial", 36).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("CPU Usage (%)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    for (i, mode) in modes.iter().enumerate() {
        if let Some(values) = modes_data.get(*mode) {
            let stats = calculate_stats(values);
            
            chart.draw_series(std::iter::once(Rectangle::new([
                (i as f64 + 0.3, 0.0),
                (i as f64 + 0.7, stats.mean)
            ], colors[i].filled())))?;
            
            // Error bars
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (i as f64 + 0.5, stats.mean - stats.std),
                (i as f64 + 0.5, stats.mean + stats.std)
            ], &BLACK)))?;
            
            // Add labels
            chart.draw_series(std::iter::once(Text::new(
                format!("{:.1}%", stats.mean),
                (i as f64 + 0.5, stats.mean + y_max * 0.05),
                ("Arial", 20).into_font(),
            )))?;
        }
    }
    
    root.present()?;
    println!("CPU usage comparison saved as {}", output_path);
    Ok(())
}

fn plot_time_breakdown(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut user_times: HashMap<String, Vec<f64>> = HashMap::new();
    let mut sys_times: HashMap<String, Vec<f64>> = HashMap::new();
    
    for run in runs {
        user_times.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.user);
        sys_times.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.sys);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes.iter()
        .flat_map(|mode| {
            let u = user_times.get(*mode)
                .and_then(|v| v.iter().fold(None, |max, &x| {
                    match max {
                        None => Some(x),
                        Some(m) => Some(f64::max(m, x))
                    }
                }))
                .unwrap_or(0.0);
            let s = sys_times.get(*mode)
                .and_then(|v| v.iter().fold(None, |max, &x| {
                    match max {
                        None => Some(x),
                        Some(m) => Some(f64::max(m, x))
                    }
                }))
                .unwrap_or(0.0);
            vec![u, s]
        })
        .fold(0f64, |a, b| f64::max(a, b)) * 1.3;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - User/System Time Breakdown", ("Arial", 32).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Time (seconds)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    for (i, mode) in modes.iter().enumerate() {
        let user_stats = user_times.get(*mode).map(|v| calculate_stats(v));
        let sys_stats = sys_times.get(*mode).map(|v| calculate_stats(v));
        
        if let (Some(user), Some(sys)) = (user_stats, sys_stats) {
            // User time bar
            chart.draw_series(std::iter::once(Rectangle::new([
                (i as f64 + 0.2, 0.0),
                (i as f64 + 0.5, user.mean)
            ], colors[i].filled())))?;
            
            // System time bar (lighter shade)
            chart.draw_series(std::iter::once(Rectangle::new([
                (i as f64 + 0.5, 0.0),
                (i as f64 + 0.8, sys.mean)
            ], colors[i].mix(0.6).filled())))?;
            
            // Labels
            chart.draw_series(std::iter::once(Text::new(
                format!("{:.3}s", user.mean),
                (i as f64 + 0.35, user.mean + y_max * 0.02),
                ("Arial", 16).into_font(),
            )))?;
            
            chart.draw_series(std::iter::once(Text::new(
                format!("{:.3}s", sys.mean),
                (i as f64 + 0.65, sys.mean + y_max * 0.02),
                ("Arial", 16).into_font(),
            )))?;
        }
    }
    
    root.present()?;
    println!("Time breakdown saved as {}", output_path);
    Ok(())
}

fn plot_speedup_comparison(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.real);
    }
    
    let run_stats = modes_data.get("run").map(|v| calculate_stats(v)).unwrap();
    let interpreter_time = run_stats.mean;
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let mut speedups = Vec::new();
    for mode in &modes {
        if let Some(values) = modes_data.get(*mode) {
            let stats = calculate_stats(values);
            speedups.push(interpreter_time / stats.mean);
        } else {
            speedups.push(0.0);
        }
    }
    
    let y_max = speedups.iter().fold(0f64, |a, &b| f64::max(a, b)) * 1.2;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - Performance Speedup", ("Arial", 32).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Speedup Factor")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    // Baseline reference line
    chart.draw_series(std::iter::once(PathElement::new(vec![
        (0.0, 1.0),
        (3.0, 1.0)
    ], ShapeStyle::from(&RGBColor(128, 128, 128)).stroke_width(2))))?;
    
    for (i, speedup) in speedups.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new([
            (i as f64 + 0.3, 0.0),
            (i as f64 + 0.7, *speedup)
        ], colors[i].filled())))?;
        
        // Add labels
        chart.draw_series(std::iter::once(Text::new(
            format!("{:.1}x", speedup),
            (i as f64 + 0.5, speedup + y_max * 0.05),
            ("Arial", 24).into_font().style(FontStyle::Bold),
        )))?;
    }
    
    root.present()?;
    println!("Speedup comparison saved as {}", output_path);
    Ok(())
}

fn plot_memory_vs_time(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let x_max = runs.iter().map(|r| r.real).fold(0f64, |a, b| f64::max(a, b)) * 1.1;
    let y_max = runs.iter().map(|r| r.max_memory_kb).fold(0f64, |a, b| f64::max(a, b)) * 1.1;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language Performance Benchmark - Memory vs Execution Time", ("Arial", 32).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..x_max, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Time (s)")
        .y_desc("Memory Usage (KB)")
        .draw()?;
    
    let colors = HashMap::from([
        ("run", RGBColor(200, 60, 30)),
        ("build", RGBColor(30, 80, 200)),
        ("jit", RGBColor(30, 150, 50)),
    ]);
    
    for (mode, color) in colors.iter() {
        let mode_data: Vec<_> = runs.iter()
            .filter(|r| r.mode == *mode)
            .map(|r| (r.real, r.max_memory_kb))
            .collect();
        
        chart.draw_series(
            mode_data.iter().map(|&(x, y)| Circle::new((x, y), 5, color.filled()))
        )?;
    }
    
    root.present()?;
    println!("Memory vs Time plot saved as {}", output_path);
    Ok(())
}

fn plot_time_distribution_violin(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.real);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes_data.values()
        .flat_map(|v| v.iter())
        .fold(0f64, |a, &b| f64::max(a, b)) * 1.2;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language - Execution Time Distribution (Violin Plot)", ("Arial", 32).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Time (seconds)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    // Draw violin plots for each mode
    for (i, mode) in modes.iter().enumerate() {
        if let Some(values) = modes_data.get(*mode) {
            let stats = calculate_stats(values);
            let x_center = i as f64 + 0.5;
            
            // Create a simple violin shape using kernel density estimation
            let num_points = 50;
            let bandwidth = stats.std * 0.4;
            
            for j in 0..num_points {
                let y = stats.min + (stats.max - stats.min) * (j as f64) / (num_points as f64);
                let mut density = 0.0;
                
                // Simple kernel density estimation
                for &val in values {
                    let u = (y - val) / bandwidth;
                    density += (-0.5 * u * u).exp() / (2.5066282746310005 * bandwidth);
                }
                density /= values.len() as f64;
                
                // Scale density to create violin width
                let width = density * 0.3 * 10.0;
                
                if width > 0.01 {
                    chart.draw_series(std::iter::once(Rectangle::new([
                        (x_center - width, y),
                        (x_center + width, y + (stats.max - stats.min) / (num_points as f64))
                    ], colors[i].mix(0.7).filled())))?;
                }
            }
            
            // Add median line
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center - 0.1, stats.median),
                (x_center + 0.1, stats.median)
            ], ShapeStyle::from(&WHITE).stroke_width(3))))?;
            
            // Add mean dot
            chart.draw_series(std::iter::once(Circle::new(
                (x_center, stats.mean),
                5,
                &WHITE,
            )))?;
        }
    }
    
    root.present()?;
    println!("Violin distribution plot saved as {}", output_path);
    Ok(())
}

fn plot_time_distribution_box(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.real);
    }
    
    let modes: Vec<&str> = vec!["run", "build", "jit"];
    let colors = vec![
        RGBColor(200, 60, 30),   // run - darker orange/red
        RGBColor(30, 80, 200),   // build - darker blue  
        RGBColor(30, 150, 50),   // jit - darker green
    ];
    
    let y_max = modes_data.values()
        .flat_map(|v| v.iter())
        .fold(0f64, |a, &b| f64::max(a, b)) * 1.2;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language - Execution Time Distribution (Box Plot)", ("Arial", 32).into_font())
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .build_cartesian_2d(0f64..3f64, 0f64..y_max)?;

    chart.configure_mesh()
        .x_desc("Execution Mode")
        .y_desc("Time (seconds)")
        .x_label_formatter(&|x| {
            match x.floor() as i32 {
                0 => "run",
                1 => "build",
                2 => "jit",
                _ => "",
            }.to_string()
        })
        .draw()?;
    
    // Draw box plots for each mode
    for (i, mode) in modes.iter().enumerate() {
        if let Some(values) = modes_data.get(*mode) {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let n = sorted.len() as f64;
            let q1 = sorted[(n * 0.25) as usize];
            let q2 = sorted[(n * 0.5) as usize]; // median
            let q3 = sorted[(n * 0.75) as usize];
            let iqr = q3 - q1;
            let whisker_low = sorted.iter()
                .find(|&&x| x >= q1 - 1.5 * iqr)
                .copied()
                .unwrap_or(sorted[0]);
            let whisker_high = sorted.iter()
                .rev()
                .find(|&&x| x <= q3 + 1.5 * iqr)
                .copied()
                .unwrap_or(sorted[sorted.len() - 1]);
            
            let x_center = i as f64 + 0.5;
            let box_width = 0.2;
            
            // Draw box
            chart.draw_series(std::iter::once(Rectangle::new([
                (x_center - box_width, q1),
                (x_center + box_width, q3)
            ], colors[i].filled())))?;
            
            // Draw median line
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center - box_width, q2),
                (x_center + box_width, q2)
            ], ShapeStyle::from(&WHITE).stroke_width(3))))?;
            
            // Draw whiskers
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center, q1),
                (x_center, whisker_low)
            ], ShapeStyle::from(&BLACK).stroke_width(2))))?;
            
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center, q3),
                (x_center, whisker_high)
            ], ShapeStyle::from(&BLACK).stroke_width(2))))?;
            
            // Draw whisker caps
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center - box_width/2.0, whisker_low),
                (x_center + box_width/2.0, whisker_low)
            ], ShapeStyle::from(&BLACK).stroke_width(2))))?;
            
            chart.draw_series(std::iter::once(PathElement::new(vec![
                (x_center - box_width/2.0, whisker_high),
                (x_center + box_width/2.0, whisker_high)
            ], ShapeStyle::from(&BLACK).stroke_width(2))))?;
            
            // Draw outliers
            for &val in values.iter() {
                if val < whisker_low || val > whisker_high {
                    chart.draw_series(std::iter::once(Circle::new(
                        (x_center, val),
                        3,
                        colors[i].filled(),
                    )))?;
                }
            }
            
            // Add mean marker
            let stats = calculate_stats(values);
            chart.draw_series(std::iter::once(Cross::new(
                (x_center, stats.mean),
                5,
                ShapeStyle::from(&WHITE).stroke_width(2),
            )))?;
        }
    }
    
    root.present()?;
    println!("Box plot distribution saved as {}", output_path);
    Ok(())
}

fn plot_time_histogram(runs: &[BenchmarkRun], output_path: &str) -> PlotResult<()> {
    let root = BitMapBackend::new(output_path, (1400, 800)).into_drawing_area();
    root.fill(&RGBColor(192, 192, 192))?;
    
    let mut all_times = Vec::new();
    let mut modes_data: HashMap<String, Vec<f64>> = HashMap::new();
    
    for run in runs {
        modes_data.entry(run.mode.clone())
            .or_insert_with(Vec::new)
            .push(run.real);
        all_times.push(run.real);
    }
    
    let colors = HashMap::from([
        ("run", RGBColor(200, 60, 30)),
        ("build", RGBColor(30, 80, 200)),
        ("jit", RGBColor(30, 150, 50)),
    ]);
    
    let x_min = all_times.iter().fold(f64::INFINITY, |a, &b| f64::min(a, b));
    let x_max = all_times.iter().fold(f64::NEG_INFINITY, |a, &b| f64::max(a, b));
    let x_range = x_max - x_min;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Kozeig Language - Execution Time Histogram", ("Arial", 32).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .right_y_label_area_size(10)
        .build_cartesian_2d(
            (x_min - x_range * 0.1)..(x_max + x_range * 0.1),
            0f64..15f64  
        )?;

    chart.configure_mesh()
        .x_desc("Time (seconds)")
        .y_desc("Frequency")
        .draw()?;
    
    // Create histograms for each mode
    for (mode, values) in modes_data.iter() {
        let color = colors.get(mode.as_str()).unwrap();
        let bin_count = 15;
        let bin_width = (x_max - x_min) / bin_count as f64;
        
        let mut bins = vec![0u32; bin_count];
        
        for &val in values {
            let bin_idx = ((val - x_min) / bin_width).floor() as usize;
            if bin_idx < bin_count {
                bins[bin_idx] += 1;
            }
        }
        
        // Draw histogram bars with slight offset for overlap
        let offset = match mode.as_str() {
            "run" => -0.01,
            "build" => 0.0,
            "jit" => 0.01,
            _ => 0.0,
        };
        
        chart.draw_series(
            bins.iter().enumerate().map(|(idx, &count)| {
                Rectangle::new([
                    (x_min + idx as f64 * bin_width + offset, 0f64),
                    (x_min + (idx + 1) as f64 * bin_width + offset, count as f64)
                ], color.mix(0.7).filled())
            })
        )?;
    }
    
    // Add legend 
    let legend_entries = vec![
        ("Interpreter", RGBColor(200, 60, 30)),
        ("Compiler", RGBColor(30, 80, 200)),
        ("JIT", RGBColor(30, 150, 50)),
    ];
    
    for (i, (label, color)) in legend_entries.iter().enumerate() {
        let x = x_max - x_range * 0.2;
        let y = 13.0 - i as f64 * 1.5;
        
        chart.draw_series(std::iter::once(Rectangle::new([
            (x, y),
            (x + x_range * 0.03, y + 0.8)
        ], color.filled())))?;
        
        chart.draw_series(std::iter::once(Text::new(
            *label,
            (x + x_range * 0.04, y + 0.4),
            ("Arial", 16).into_font(),
        )))?;
    }
    
    root.present()?;
    println!("Histogram distribution saved as {}", output_path);
    Ok(())
}

pub fn main() -> PlotResult<()> {
    let args: Vec<String> = std::env::args().collect();
    let csv_file = if args.len() > 1 {
        &args[1]
    } else {
        "benchmark_results.csv"
    };
    
    let runs = read_benchmark_data(Path::new(csv_file))?;
    
    // Create all plots
    plot_execution_time(&runs, "benchmark_time_all.png")?;
    plot_memory_usage(&runs, "benchmark_memory.png")?;
    plot_cpu_usage(&runs, "benchmark_cpu.png")?;
    plot_time_breakdown(&runs, "benchmark_time_breakdown.png")?;
    plot_speedup_comparison(&runs, "speedup_comparison.png")?;
    plot_memory_vs_time(&runs, "memory_vs_time.png")?;
    
    // Additional distribution plots
    plot_time_distribution_violin(&runs, "benchmark_distribution_violin.png")?;
    plot_time_distribution_box(&runs, "benchmark_distribution_box.png")?;
    plot_time_histogram(&runs, "benchmark_histogram.png")?;
    
    println!("All benchmark plots generated successfully!");
    Ok(())
}