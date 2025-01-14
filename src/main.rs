use rustybench::{run_cpu_benchmark, run_gpu_benchmark};
use colored::*;
use sysinfo::System;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustybench")]
#[command(about = "A CPU and GPU benchmark tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run CPU benchmarks")]
    Cpu {
        #[arg(long, help = "Run quick 5s tests instead of 20s")]
        quick: bool,
    },
    #[command(about = "Run GPU benchmarks")]
    Gpu,
    #[command(about = "Run all benchmarks")]
    All {
        #[arg(long, help = "Run quick 5s tests instead of 20s")]
        quick: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "\nSystem Information".bright_blue().bold());
    println!("CPU: {}", sys.cpus()[0].name());
    println!("Cores: {}", num_cpus::get());
    println!("Memory: {} GB", sys.total_memory() / 1024 / 1024 / 1024);

    match cli.command {
        Commands::Cpu { quick } => {
            run_cpu_bench(quick).await;
        }
        Commands::Gpu => {
            run_gpu_bench().await;
        }
        Commands::All { quick } => {
            run_cpu_bench(quick).await;
            run_gpu_bench().await;
        }
    }
}

async fn run_cpu_bench(quick: bool) {
    let duration = if quick { "5s" } else { "20s" };
    println!("\n{}", format!("Starting CPU Benchmark ({} per test)", duration).bright_green().bold());
    let results = run_cpu_benchmark(quick).await;

    println!("\n{}", "CPU Results:".bright_yellow().bold());
    for result in results {
        println!("\n{}", result.benchmark_type.bright_cyan());
        println!("Time: {:.2?}", result.duration);
        println!("Ops/s: {:.2e}", result.operations_per_second);
        println!("Score: {:.2}", result.score);
    }
}

async fn run_gpu_bench() {
    println!("\n{}", "Starting GPU Benchmark".bright_green().bold());
    match run_gpu_benchmark().await {
        Some(result) => {
            println!("\n{}", "GPU Results:".bright_yellow().bold());
            println!("Device: {}", result.device_name.bright_cyan());
            println!("Type: {}", result.device_type);
            println!("Time: {:.2?}", result.duration);
            println!("Compute Score: {:.2} MOps/s", result.compute_score);
            println!("Memory Score: {:.2} MB/s", result.memory_score);
        }
        None => {
            println!("\n{}", "No compatible GPU found".bright_red());
        }
    }
}