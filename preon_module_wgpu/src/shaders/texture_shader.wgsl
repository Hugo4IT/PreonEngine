struct Transform {
    transform: vec2<f32>,
};

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> global: Transform;

@vertex
fn vert_main(
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) z_index: f32,
    @location(3) dimensions: vec4<f32>,
    @location(4) uv_dimensions: vec4<f32>
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vec2<f32>(
        uv_dimensions.x + uv_dimensions.z * tex_coords.x,
        uv_dimensions.y + uv_dimensions.w * tex_coords.y,
    );

    let rp = dimensions.xy;
    let rs = dimensions.zw;

    out.position = vec4<f32>(
        global.transform * rp * vec2<f32>(1.0, -1.0) + vec2<f32>(-1.0, 1.0) +
        global.transform * rs * position,
        z_index,
        1.0
    );

    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var textureSampler: sampler;

@fragment
fn frag_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return textureSample(texture, textureSampler, in.uv);
}
