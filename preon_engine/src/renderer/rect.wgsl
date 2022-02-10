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

struct Screen {
    s_size: vec2<f32>;
};

[[group(0), binding(0)]]
var<uniform> screen: Screen;

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;

    let pivot_offset = vec3<f32>(instance.size.x * 0.5, -instance.size.y * 0.5, 1.0);
    let position = model.v_position * vec3<f32>(instance.size, 1.0) + pivot_offset + instance.i_position;
    let screen_position = (position - vec3<f32>(screen.s_size.x * 0.5, -screen.s_size.y * 0.5, 0.0)) * vec3<f32>(2.0, 2.0, 1.0);
    let transformed_position = screen_position / vec3<f32>(screen.s_size, 1.0);

    out.clip_position = vec4<f32>(transformed_position, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}