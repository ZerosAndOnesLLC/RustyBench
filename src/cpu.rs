use rayon::prelude::*;
use std::time::{Duration, Instant};
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

pub async fn run_benchmark(quick: bool) -> Vec<BenchmarkResult> {
    let cpu_count = num_cpus::get();
    let m = MultiProgress::new();
    let workloads: Vec<(&str, fn(u64) -> u64)> = vec![
        ("Floating Point", test_floating_point),
        ("Integer", test_integer),
        ("Memory", test_memory),
        ("Prime", test_prime),
    ];

    let mut results = Vec::new();
    let test_duration = if quick {
        Duration::from_secs(3)
    } else {
        Duration::from_secs(10)
    };

    for (name, workload) in workloads {
        let pb = m.add(ProgressBar::new(test_duration.as_secs()));
        pb.set_style(ProgressStyle::default_bar()
            .template("\n{spinner:.green} {msg}\n[{bar:40.cyan/blue}] {percent}% ({eta})")
            .unwrap());
        pb.set_message(name.to_string());

        let start = Instant::now();
        let batch_size = 2000;
        let mut total_ops = 0u64;
        let mut result = 0u64;

        while start.elapsed() < test_duration {
            result = result.wrapping_add((0..cpu_count).into_par_iter()
                .map(|_| workload(batch_size))
                .sum::<u64>());
            total_ops += batch_size * cpu_count as u64;
            pb.set_position(start.elapsed().as_secs());
        }

        let duration = start.elapsed();
        pb.finish_with_message(format!("{} complete", name));

        results.push(BenchmarkResult {
            duration,
            operations_per_second: total_ops as f64 / duration.as_secs_f64(),
            cpu_count,
            benchmark_type: name.to_string(),
            score: calculate_score(result, duration, total_ops),
        });
    }

    m.clear().unwrap();
    results
}

fn test_floating_point(iterations: u64) -> u64 {
    let mut x = 1.0f64;
    let mut y = 2.0f64;
    let mut z = 3.0f64;
    let mut w = 4.0f64;
    let mut sum = 0u64;
    
    for _ in 0..iterations {
        for _ in 0..2000 {
            x = (x * 1.1).sqrt().sin().exp();
            y = (y * 1.2).cos().tan().exp();
            z = (z * 1.3).sin().cos().ln();
            w = (w * 1.4).tan().exp().sqrt();
            sum = sum.wrapping_add(((x + y + z + w).abs()) as u64);
            
            x = y + 0.1;
            y = z + 0.2;
            z = w + 0.3;
            w = x + 0.4;
        }
    }
    sum
}

fn test_integer(iterations: u64) -> u64 {
    let mut a = 1u64;
    let mut b = 2u64;
    let mut c = 3u64;
    let mut d = 4u64;
    let mut sum = 0u64;
    
    for i in 0..iterations {
        for _ in 0..2000 {
            a = a.wrapping_mul(i.wrapping_add(1)).rotate_left(3);
            b = b.wrapping_add(a).rotate_right(2);
            c = c.wrapping_mul(b).rotate_left(1);
            d = d.wrapping_add(c).rotate_right(3);
            
            sum = sum.wrapping_add(a ^ b ^ c ^ d);
            
            std::mem::swap(&mut a, &mut d);
            std::mem::swap(&mut b, &mut c);
        }
    }
    sum
}

fn test_memory(iterations: u64) -> u64 {
    let size = 20_000_000;
    let mut vec = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    
    for _ in 0..size {
        vec.push(rng.gen::<u64>());
    }
    
    let mut sum = 0u64;
    let mut last_idx = 0;
    for _ in 0..iterations {
        for _ in 0..200 {
            let idx = rng.gen_range(0..size);
            sum = sum.wrapping_add(vec[idx]);
            vec[last_idx] = sum;
            last_idx = idx;
        }
    }
    sum
}

fn test_prime(iterations: u64) -> u64 {
    let mut count = 0;
    let chunk_size = 2000;
    for i in 0..iterations {
        for j in 0..chunk_size {
            let n = i * chunk_size + j;
            if is_prime(n) {
                count += 1;
            }
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

fn calculate_score(_result: u64, duration: Duration, total_ops: u64) -> f64 {
    let base_score = 10000.0;
    let cpu_count = num_cpus::get() as f64;
    let ops_per_second = total_ops as f64 / duration.as_secs_f64();
    let per_core_ops = ops_per_second / cpu_count;
    base_score * ((per_core_ops / 1_000_000.0).abs() + 1.0).log10() * (cpu_count / 2.0).sqrt()
}