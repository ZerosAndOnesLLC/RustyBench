# RustyBench

A multi-threaded CPU and GPU benchmark tool written in Rust.

## Features

### CPU Tests (20s each)
- Floating Point Operations
- Integer Operations
- Memory Access
- Prime Number Calculations
- Utilizes all available CPU cores

### GPU Tests
- Compute shader performance test
- Memory transfer speed test
- Cross-platform GPU support via wgpu

## Requirements

- Rust 1.70+
- Compatible GPU with Vulkan, Metal, or DirectX 12

## Installation

```bash
git clone https://github.com/yourusername/RustyBench
cd RustyBench
cargo build --release
```

## Usage

```bash
./target/release/rustybench [COMMAND]

Commands:
  cpu   Run CPU benchmarks
  gpu   Run GPU benchmarks
  all   Run all benchmarks
  help  Print help

Options:
  --quick  Run shorter CPU tests (5s instead of 20s)
```

## Example Output

```
System Information
CPU: AMD Ryzen 9 5950X
Cores: 16
Memory: 32 GB

CPU Results:
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

GPU Results:
Device: NVIDIA GeForce RTX 3080
Type: Discrete
Time: 5.32s
Compute Score: 1234.56 MOps/s
Memory Score: 15678.90 MB/s
```

## License

MIT License

## Authors
- MacK42 on github