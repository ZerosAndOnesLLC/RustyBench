use rustybench::{run_cpu_benchmark, run_gpu_benchmark};
use colored::*;
use sysinfo::System;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustybench")]
#[command(about = "A CPU and GPU benchmark tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run CPU benchmarks")]
    Cpu {
        #[arg(long, help = "Run quick 3s tests instead of 10s")]
        quick: bool,
    },
    #[command(about = "Run GPU benchmarks")]
    Gpu,
    #[command(about = "Run all benchmarks")]
    All {
        #[arg(long, help = "Run quick 3s tests instead of 10s")]
        quick: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut sys = System::new_all();
    sys.refresh_all();

    print_system_info(&sys);

    match cli.command {
        Some(Commands::Cpu { quick }) => {
            run_cpu_bench(quick).await;
        }
        Some(Commands::Gpu) => {
            run_gpu_bench().await;
        }
        Some(Commands::All { quick }) => {
            run_cpu_bench(quick).await;
            run_gpu_bench().await;
        }
        None => {
            // Default to All with no quick mode
            run_cpu_bench(false).await;
            run_gpu_bench().await;
        }
    }
}

fn print_system_info(sys: &System) {
    println!("{}", "\nSystem Information".bright_blue().bold());
    
    // Get detailed CPU information
    let cpu = &sys.cpus()[0];
    println!("CPU: {} @ {:.2} GHz", cpu.brand(), cpu.frequency() as f32 / 1000.0);
    println!("CPU Vendor: {}", cpu.vendor_id());
    println!("Physical Cores: {}", num_cpus::get_physical());
    println!("Logical Cores: {}", num_cpus::get());
    println!("Memory: {} GB", sys.total_memory() / 1024 / 1024 / 1024);
    
    // Optional: Add CPU cache information if available
    #[cfg(target_os = "linux")]
    {
        if let Ok(cache_size) = std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index3/size") {
            println!("L3 Cache: {}", cache_size.trim());
        }
    }
}

async fn run_cpu_bench(quick: bool) {
    let duration = if quick { "3s" } else { "10s" };
    println!("\n{}", format!("Starting CPU Benchmark ({} per test)", duration).bright_green().bold());
    let results = run_cpu_benchmark(quick).await;
    print_cpu_results(&results);
}

async fn run_gpu_bench() {
    println!("\n{}", "Starting GPU Benchmark".bright_green().bold());
    match run_gpu_benchmark().await {
        Some(result) => print_gpu_results(&result),
        None => println!("\n{}", "No compatible GPU found".bright_red()),
    }
}

fn print_cpu_results(results: &[rustybench::CPUBenchmarkResult]) {
    println!("\n{}", "CPU Results:".bright_yellow().bold());
    for result in results {
        let ops = result.operations_per_second as u64;
        let ops_formatted = format!("{}", ops)
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(",")
            .chars()
            .rev()
            .collect::<String>();

        println!("\n{}", result.benchmark_type.bright_cyan().bold());
        println!("{:<12} {:>10.2?}", "Time:", result.duration);
        println!("{:<12} {}", "Ops/s:", format!("{:>10} ops/s", ops_formatted));
        println!("{:<12} {:>10.2}", "Score:", result.score);
    }
}

fn print_gpu_results(result: &rustybench::GPUBenchmarkResult) {
    println!("\n{}", "GPU Results:".bright_yellow().bold());
    println!("{:<12} {}", "Device:", result.device_name.bright_cyan());
    println!("{:<12} {}", "Type:", result.device_type);
    println!("{:<12} {:.2?}", "Time:", result.duration);
    println!("{:<12} {:.2} MOps/s", "Compute:", result.compute_score);
    println!("{:<12} {:.2} MB/s", "Memory:", result.memory_score);
}