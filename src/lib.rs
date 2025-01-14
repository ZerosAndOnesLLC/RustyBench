pub mod cpu;
pub mod gpu;

pub use crate::cpu::BenchmarkResult as CPUBenchmarkResult;
pub use crate::gpu::GPUBenchmarkResult;
pub use crate::cpu::run_benchmark as run_cpu_benchmark;
pub use crate::gpu::run_benchmark as run_gpu_benchmark;