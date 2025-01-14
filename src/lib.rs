use rayon::prelude::*;
use std::time::{Duration, Instant};
use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;

#[derive(Debug)]
pub struct BenchmarkResult {
    pub duration: Duration,
    pub operations_per_second: f64,
    pub cpu_count: usize,
    pub benchmark_type: String,
    pub score: f64,
}

pub async fn run_benchmark() -> Vec<BenchmarkResult> {
    let cpu_count = num_cpus::get();
    let m = MultiProgress::new();
    let workloads: Vec<(&str, fn(u64) -> u64)> = vec![
        ("Floating Point", test_floating_point),
        ("Integer", test_integer),
        ("Memory", test_memory),
        ("Prime", test_prime),
    ];

    let mut results = Vec::new();
    let test_duration = Duration::from_secs(20);

    for (name, workload) in workloads {
        let pb = m.add(ProgressBar::new_spinner());
        pb.set_style(ProgressStyle::default_spinner()
            .template("{spinner:.green} Testing {msg}")
            .unwrap());
        pb.set_message(name.to_string());

        let start = Instant::now();
        let mut iterations = 0u64;
        let mut result = 0u64;

        while start.elapsed() < test_duration {
            result += (0..cpu_count).into_par_iter()
                .map(|_| workload(1_000_000))
                .sum::<u64>();
            iterations += cpu_count as u64 * 1_000_000;
            pb.inc(1);
        }

        let duration = start.elapsed();
        pb.finish_with_message(format!("{} complete", name));

        results.push(BenchmarkResult {
            duration,
            operations_per_second: iterations as f64 / duration.as_secs_f64(),
            cpu_count,
            benchmark_type: name.to_string(),
            score: calculate_score(result, duration),
        });
    }

    m.clear().unwrap();
    results
}

fn test_floating_point(iterations: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iterations {
        let x = (i as f64).sqrt().sin().cos().exp();
        sum += x.abs() as u64;
    }
    sum
}

fn test_integer(iterations: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iterations {
        sum = sum.wrapping_mul(i).wrapping_add(i);
    }
    sum
}

fn test_memory(iterations: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(1_000_000);
    for _ in 0..iterations {
        vec.push(rng.gen::<u64>());
        if vec.len() > 1_000_000 {
            vec.clear();
        }
    }
    vec.iter().sum()
}

fn test_prime(iterations: u64) -> u64 {
    let mut count = 0;
    for i in 0..iterations {
        if is_prime(i) {
            count += 1;
        }
    }
    count
}

fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

fn calculate_score(result: u64, duration: Duration) -> f64 {
    let base_score = 1000.0;
    let operations_per_second = result as f64 / duration.as_secs_f64();
    base_score * (operations_per_second / 1_000_000.0).log10()
}