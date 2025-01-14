use bench_rust::run_benchmark;
use colored::*;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("{}", "System Information".bright_blue().bold());
    println!("CPU: {}", sys.cpus()[0].name());
    println!("Cores: {}", num_cpus::get());
    println!("Memory: {} GB", sys.total_memory() / 1024 / 1024 / 1024);
    
    println!("\n{}", "Starting 20-second Benchmark".bright_green().bold());
    let results = run_benchmark().await;

    println!("\n{}", "Results:".bright_yellow().bold());
    for result in results {
        println!("\n{}", result.benchmark_type.bright_cyan());
        println!("Time: {:.2?}", result.duration);
        println!("Ops/s: {:.2e}", result.operations_per_second);
        println!("Score: {:.2}", result.score);
    }
}