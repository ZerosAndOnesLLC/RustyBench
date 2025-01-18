# RustyBench 🚀

A high-performance CPU and GPU benchmarking tool written in Rust. RustyBench provides comprehensive system performance metrics through various computational tests and memory operations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust Version](https://img.shields.io/badge/rust-2021_edition-blue.svg)

## Features

- 🔥 Multi-threaded CPU benchmarks
- 💫 GPU compute and memory benchmarks using wgpu
- 📊 Detailed performance metrics and scoring
- 🎯 Quick test mode for rapid system assessment
- 🌈 Beautiful colored console output
- 💻 Cross-platform support (Windows, Linux, ARM)

## CPU Benchmarks

RustyBench performs the following CPU tests:

- **Floating Point**: Complex floating-point operations (sin, cos, tan, exp)
- **Integer**: Intensive integer arithmetic with bit manipulation
- **Memory**: Random access patterns and memory bandwidth testing
- **Prime**: Prime number calculation for computational performance

## GPU Benchmarks

The GPU benchmark suite includes:

- **Compute Performance**: Parallel floating-point operations using compute shaders
- **Memory Bandwidth**: High-throughput memory transfer tests
- **Device Information**: Detailed GPU capabilities reporting

## System Requirements

### Minimum Requirements
- Rust 1.70 or higher
- 2GB RAM
- x86_64 or ARM64 processor
- GPU with Vulkan, Metal, or DirectX 12 support

### Supported Platforms
- Windows 10/11 (x86_64)
- Ubuntu 24.04 (x86_64)
- Ubuntu 24.04 ARM64 (tested on Raspberry Pi 5)

## Installation

1. Ensure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository:
```bash
git clone https://github.com/yourusername/rustybench.git
cd rustybench
```

3. Build the project:
```bash
cargo build --release
```

## Usage

RustyBench supports several command-line options:

```bash
# Run all benchmarks (CPU + GPU)
./rustybench

# Run only CPU benchmarks
./rustybench cpu

# Run quick CPU benchmarks (3s instead of 10s per test)
./rustybench cpu --quick

# Run only GPU benchmarks
./rustybench gpu

# Run all benchmarks in quick mode
./rustybench all --quick
```

## Sample Output

```
System Information
CPU: CPU 1
Cores: 24

CPU Results:

Floating Point
Time:            14.86s
Ops/s:           32,303 ops/s
Score:            20.24

Integer
Time:            10.47s
Ops/s:          412,595 ops/s
Score:           256.44

Memory
Time:            10.49s
Ops/s:          343,305 ops/s
Score:           213.68

Prime
Time:            12.72s
Ops/s:           56,609 ops/s
Score:            35.44

Starting GPU Benchmark

GPU Results:
Device:      NVIDIA GeForce RTX 3090
Type:        DiscreteGpu
Time:        97.93ms
Compute:     4397.84 MOps/s
Memory:      3962848.30 MB/s
```

## Understanding the Results

- **Ops/s**: Operations per second - higher is better
- **Score**: Normalized score accounting for CPU cores and operation complexity
- **MOps/s**: Million operations per second for GPU compute
- **MB/s**: Memory bandwidth in megabytes per second

## Building from Source

### Windows Prerequisites
- Visual Studio Build Tools 2019 or newer
- Windows SDK 10.0.19041.0 or newer

### Ubuntu Prerequisites
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libx11-dev libasound2-dev libudev-dev
```

### Raspberry Pi 5 Prerequisites
```bash
sudo apt-get update
sudo apt-get install build-essential pkg-config libraspberrypi-dev
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [wgpu](https://github.com/gfx-rs/wgpu) for GPU acceleration
- [rayon](https://github.com/rayon-rs/rayon) for CPU parallelization
- [clap](https://github.com/clap-rs/clap) for command-line argument parsing

## Contact

Project Link: [https://github.com/yourusername/rustybench](https://github.com/yourusername/rustybench)

Please ⭐️ this repository if you found it helpful!