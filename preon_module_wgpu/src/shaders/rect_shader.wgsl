struct Transform {
    transform: vec2<f32>,
};

struct VertexOutput {
    @location(0) ratio: f32,
    @location(1) radius: vec4<f32>,
    @location(2) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> global: Transform;

@vertex
fn vert_main(
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) z_index: f32,
    @location(3) radius: vec4<f32>,
    @location(4) dimensions: vec4<f32>,
    @location(5) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.ratio = dimensions.z / dimensions.w;
    out.radius = radius;
    out.color = color;
    
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

@fragment
fn frag_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}
