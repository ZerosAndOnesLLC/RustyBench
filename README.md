# CPU Benchmark

A multi-threaded CPU benchmark tool written in Rust that tests different aspects of CPU performance.

## Features

- 20-second stress tests for:
  - Floating Point Operations
  - Integer Operations
  - Memory Access
  - Prime Number Calculations
- Utilizes all available CPU cores
- Real-time progress tracking
- System information display
- Performance scoring system

## Installation

```bash
git clone https://github.com/yourusername/cpu-benchmark
cd cpu-benchmark
cargo build --release
```

## Usage

Run the benchmark:
```bash
cargo run --release
```

The benchmark will run for 20 seconds per test, utilizing all available CPU cores.

## Output Example

```
System Information
CPU: AMD Ryzen 9 5950X
Cores: 16
Memory: 32 GB

Starting 20-second Benchmark

Results:
Floating Point
Time: 20.00s
Ops/s: 1.23e9
Score: 892.45

Integer
Time: 20.00s
Ops/s: 2.45e9
Score: 945.32

Memory
Time: 20.00s
Ops/s: 8.56e8
Score: 823.67

Prime
Time: 20.00s
Ops/s: 5.67e8
Score: 785.91
```

## Requirements

- Rust 1.70 or higher
- Cargo

## License

MIT License

## Authors
- MacK42 on github