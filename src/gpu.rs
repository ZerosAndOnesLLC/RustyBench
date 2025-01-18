use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

const MIN_TEST_DURATION: Duration = Duration::from_secs(3);
const COMPUTE_ITERATIONS: u32 = 100;
const MEMORY_ITERATIONS: u32 = 50;
const WORKGROUP_SIZE: u32 = 256;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ComputeData {
    data: [f32; 4],
}

#[derive(Debug)]
pub struct GPUBenchmarkResult {
    pub device_name: String,
    pub device_type: String,
    pub compute_score: f64,
    pub memory_score: f64,
    pub duration: Duration,
}

// More complex compute shader with multiple operations and memory access patterns
const COMPUTE_SHADER: &str = r#"
    @group(0) @binding(0) var<storage, read_write> data: array<vec4<f32>>;

    fn complex_math(v: vec4<f32>) -> vec4<f32> {
        var result = v;
        
        // Multiple trigonometric operations
        result = sin(result) * 0.5 + cos(result) * 0.5;
        result = tan(result) * 0.3 + result * 0.7;
        
        // Exponential and logarithmic operations
        result = exp(result * 0.1) + log(abs(result) + 1.0);
        
        // Matrix multiplication simulation
        let mat = mat4x4<f32>(
            vec4<f32>(1.1, 0.2, 0.3, 0.4),
            vec4<f32>(0.2, 1.2, 0.3, 0.4),
            vec4<f32>(0.3, 0.2, 1.3, 0.4),
            vec4<f32>(0.4, 0.2, 0.3, 1.4)
        );
        result = mat * result;
        
        return result;
    }

    @compute @workgroup_size(256)
    fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
        let index = global_id.x;
        if (index >= arrayLength(&data)) {
            return;
        }
        
        var value = data[index];
        
        // Multiple passes of complex computation
        for (var i = 0u; i < 50u; i = i + 1u) {
            value = complex_math(value);
            // Add some branching for more realistic workload
            if (value.x > 0.0) {
                value = value * 1.1;
            } else {
                value = value * 0.9;
            }
        }
        
        data[index] = value;
    }
"#;

pub async fn run_benchmark() -> Option<GPUBenchmarkResult> {
    let instance = wgpu::Instance::default();
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    }).await?;

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
            memory_hints: Default::default(),
        },
        None,
    ).await.ok()?;

    let info = adapter.get_info();
    let device_name = info.name;
    let device_type = format!("{:?}", info.device_type);

    let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(COMPUTE_SHADER.into()),
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: None,
        module: &compute_shader,
        entry_point: Some("main"),
        cache: Default::default(),
        compilation_options: Default::default(),
    });

    let start = Instant::now();
    let data_size = 2 * 1024 * 1024; // 2M elements for more workload
    
    // Initialize with varying data for more realistic testing
    let input_data: Vec<ComputeData> = (0..data_size)
        .map(|i| ComputeData { 
            data: [
                (i as f32).sin() * 0.5,
                (i as f32).cos() * 0.5,
                (i as f32 * 0.5).tan() * 0.3,
                i as f32 * 0.1
            ] 
        })
        .collect();

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: (std::mem::size_of::<ComputeData>() * data_size) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Compute benchmark with minimum duration
    let compute_start = Instant::now();
    let mut compute_iterations = 0;
    while compute_start.elapsed() < MIN_TEST_DURATION {
        for _ in 0..COMPUTE_ITERATIONS {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { 
                    label: None,
                    timestamp_writes: None,
                });
                compute_pass.set_pipeline(&compute_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);
                compute_pass.dispatch_workgroups(data_size as u32 / WORKGROUP_SIZE, 1, 1);
            }
            encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, storage_buffer.size());
            queue.submit(Some(encoder.finish()));
            compute_iterations += 1;
        }
    }
    let compute_time = compute_start.elapsed();
    device.poll(wgpu::Maintain::Wait);

    // Memory benchmark with larger datasets and minimum duration
    let memory_bench_size = 512 * 1024 * 1024; // 512MB for more thorough testing
    let memory_data: Vec<u8> = (0..memory_bench_size).map(|i| (i % 256) as u8).collect();
    
    let memory_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Memory Benchmark Buffer"),
        contents: &memory_data,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let memory_output = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Memory Output Buffer"),
        size: memory_bench_size as u64,
        usage: wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let memory_start = Instant::now();
    let mut memory_iterations = 0;
    while memory_start.elapsed() < MIN_TEST_DURATION {
        for _ in 0..MEMORY_ITERATIONS {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            encoder.copy_buffer_to_buffer(&memory_buffer, 0, &memory_output, 0, memory_bench_size as u64);
            queue.submit(Some(encoder.finish()));
            memory_iterations += 1;
        }
        device.poll(wgpu::Maintain::Wait);
    }
    let memory_time = memory_start.elapsed();

    // Calculate scores based on total operations and data processed
    let compute_ops = (data_size as u64 * compute_iterations as u64 * 50) as f64; // 50 is the inner loop count in shader
    let memory_bytes = (memory_bench_size as u64 * memory_iterations as u64) as f64;
    
    let compute_score = compute_ops / compute_time.as_secs_f64() / 1_000_000.0;
    let memory_score = memory_bytes / memory_time.as_secs_f64() / (1024.0 * 1024.0);

    Some(GPUBenchmarkResult {
        device_name,
        device_type,
        compute_score,
        memory_score,
        duration: start.elapsed(),
    })
}