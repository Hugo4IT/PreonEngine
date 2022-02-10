struct VertexInput {
    [[location(0)]] v_position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct InstanceInput {
    [[location(5)]] i_position: vec3<f32>;
    [[location(6)]] size: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(
        (model.v_position * vec3<f32>(instance.size, 1.0) // Sizing
             + vec3<f32>(instance.size.x * 0.5, -instance.size.y * 0.5, 1.0) // Set object pivot to top-left
        ) + instance.i_position - vec3<f32>(1.0, -1.0, 0.0), 1.0); // Apply position and set screen pivot to top-left
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}