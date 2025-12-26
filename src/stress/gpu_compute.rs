// GPU Compute Stress Test using wgpu
// Runs actual GPU compute workload to stress test GPU

#[cfg(feature = "gpu-compute")]
use std::time::Instant;

#[cfg(feature = "gpu-compute")]
use wgpu::util::DeviceExt;

/// Result of GPU compute stress test
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GpuComputeResult {
    pub gpu_name: String,
    pub backend: String,
    pub frames_dispatched: u32,
}

/// Run GPU compute stress test using wgpu
/// Returns Ok with result if successful, Err with message if GPU not available
#[cfg(feature = "gpu-compute")]
pub async fn run_gpu_compute_stress(
    duration_secs: u64,
    show_progress: bool,
) -> Result<GpuComputeResult, String> {
    // 1. Initialize wgpu instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // 2. Request GPU adapter (HighPerformance for discrete GPU)
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await.ok_or("No GPU adapter found with compute support".to_string())?;

    let info = adapter.get_info();
    let gpu_name = info.name.to_string();
    let backend = format!("{:?}", info.backend);

    // 3. Create device and queue
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ).await.map_err(|e| format!("Failed to create GPU device: {}", e))?;

    // 4. Prepare data buffer - adjust size based on duration
    // Quick: 1M elements (~4MB), Normal: 10M elements (~40MB)
    let data_size = if duration_secs <= 15 {
        1_000_000
    } else {
        10_000_000
    };

    let input_data = vec![1.23f32; data_size];

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("GPU Stress Buffer"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    // 5. Load compute shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("GPU Stress Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("gpu_stress.wgsl").into()),
    });

    // 6. Create compute pipeline
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("GPU Stress Pipeline"),
        layout: None,
        module: &shader,
        entry_point: "gpu_stress_main",
        compilation_options: Default::default(),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // 7. Main stress loop
    let start_time = Instant::now();
    let mut frames_dispatched = 0u32;
    let mut last_update_time = 0.0f32;

    while start_time.elapsed().as_secs() < duration_secs {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("GPU Stress Encoder"),
        });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);

            // Dispatch workgroups (64 threads per workgroup)
            // Max workgroup count per dimension is 65535
            // Use 2D dispatch to handle large data sizes
            let workgroups_1d = (data_size as u32 / 64) + 1;
            let x_size = workgroups_1d.min(65535);
            let y_size = ((workgroups_1d + 65534) / 65535).min(65535);

            cpass.dispatch_workgroups(x_size, y_size, 1);
        }

        queue.submit(Some(encoder.finish()));

        // Force GPU to execute
        device.poll(wgpu::Maintain::Wait);

        frames_dispatched += 1;

        // Progress display - update every 0.5 seconds or every 100 frames
        if show_progress {
            let elapsed = start_time.elapsed().as_secs_f32();
            let should_update = frames_dispatched % 100 == 0
                || (elapsed - last_update_time) >= 0.5;

            if should_update {
                last_update_time = elapsed;
                let percent = ((elapsed / duration_secs as f32) * 100.0).min(100.0) as u8;
                // Use same format as CPU: █ for filled, ░ for empty
                let filled = (percent as usize * 14 / 100).min(14);
                let empty = 14 - filled;
                let bar = format!("{}{}{}{}",
                    "\x1b[32m", "█".repeat(filled), "\x1b[90m", "░".repeat(empty));
                print!("\r⏳ GPU: [{}\x1b[0m] {}% | {} frames",
                       bar, percent, frames_dispatched);
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }
    }

    // Clear the progress line when done (reset color first to avoid color bleeding)
    if show_progress {
        print!("\x1b[0m\r\x1b[2K");  // Reset color, then clear line
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    Ok(GpuComputeResult {
        gpu_name,
        backend,
        frames_dispatched,
    })
}

/// Synchronous wrapper for async GPU compute
#[cfg(feature = "gpu-compute")]
pub fn run_gpu_compute_stress_sync(
    duration_secs: u64,
    show_progress: bool,
) -> Result<GpuComputeResult, String> {
    pollster::block_on(run_gpu_compute_stress(duration_secs, show_progress))
}

/// Stub implementation when gpu-compute feature is disabled
#[cfg(not(feature = "gpu-compute"))]
pub fn run_gpu_compute_stress_sync(
    _duration_secs: u64,
    _show_progress: bool,
) -> Result<GpuComputeResult, String> {
    Err("GPU compute stress test not enabled. Build with --features gpu-compute".to_string())
}

// When gpu-compute feature is disabled, use the same struct definition
// but it's already defined above, so no need to redefine here
