// GPU Compute Stress Test Shader
// Runs heavy ALU operations to maximize GPU load

@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

// Pseudo-random hash to prevent GPU optimization
fn hash(value: u32) -> f32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return f32(state) / 4294967295.0;
}

@compute @workgroup_size(64, 1, 1)
fn gpu_stress_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Handle 2D dispatch - flatten to 1D index
    let index = global_id.y * 65535u * 64u + global_id.x;

    if (index >= arrayLength(&data)) {
        return;
    }

    var val = data[index];

    // Heavy compute loop - constant iteration count for WGSL compliance
    // Using fma, trig, and sqrt to stress ALU units
    // Unrolled loops for maximum GPU load
    for (var i = 0u; i < 100u; i++) {
        // Fused Multiply-Add (FMA) - heavy on compute units
        val = fma(val, 1.00001, 0.00002);
        val = fma(val, 1.00002, 0.00003);
        val = fma(val, 1.00003, 0.00004);
        val = fma(val, 1.00004, 0.00005);
        val = fma(val, 1.00005, 0.00006);

        // Trigonometric functions - very expensive
        val = val + sin(val) * cos(val);
        val = val + sin(val + 0.1) * cos(val + 0.2);
        val = val + sin(val + 0.3) * cos(val + 0.4);
        val = val + sin(val + 0.5) * cos(val + 0.6);
        val = val + sin(val + 0.7) * cos(val + 0.8);

        // Square root - additional compute
        val = sqrt(abs(val));
        val = sqrt(abs(val) + 0.01);
        val = sqrt(abs(val) + 0.02);
        val = sqrt(abs(val) + 0.03);
        val = sqrt(abs(val) + 0.04);
    }

    data[index] = val;
}
