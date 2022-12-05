struct Globals {
    pixel_size: vec2<f32>,
};

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) z_index: f32,
    @location(3) rect: vec4<f32>,
    @location(4) radius: vec4<f32>,
    @location(5) color: vec4<f32>,
    @location(6) uv_cutout: vec4<f32>,
};

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) ratio: f32,
    @location(2) radius: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) use_texture: f32,
    @location(5) local_uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> global: Globals;

@vertex
fn vert_main(
    vert: VertexInput,
    inst: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vec2<f32>(
        inst.uv_cutout.x + inst.uv_cutout.z * vert.tex_coords.x,
        inst.uv_cutout.y + inst.uv_cutout.w * vert.tex_coords.y,
    );
    out.local_uv = vert.tex_coords;

    let rect_position = inst.rect.xy;
    let rect_size = inst.rect.zw;

    out.position = vec4<f32>(
        global.pixel_size * rect_position * vec2<f32>(1.0, -1.0) + vec2<f32>(-1.0, 1.0) +
        global.pixel_size * rect_size * vert.position,
        inst.z_index,
        1.0
    );

    // if inst.uv_cutout.x < 0.0 { 0.0 } else { 1.0 }
    out.use_texture = 1.0 - max(sign(-inst.uv_cutout.x), 0.0);
    out.radius = inst.radius;
    out.color = inst.color;

    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn frag_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let texture = textureSample(texture, texture_sampler, in.uv);
    let color = vec4<f32>(in.color);

    let mix = color + texture * in.use_texture;

    let corner_mask_top_left =     (max(sign(0.5-in.local_uv.x), 0.0))       * (max(sign(0.5-in.local_uv.y), 0.0));
    let corner_mask_top_right =    (1.0 - max(sign(0.5-in.local_uv.x), 0.0)) * (max(sign(0.5-in.local_uv.y), 0.0));
    let corner_mask_bottom_left =  (max(sign(0.5-in.local_uv.x), 0.0))       * (1.0 - max(sign(0.5-in.local_uv.y), 0.0));
    let corner_mask_bottom_right = (1.0 - max(sign(0.5-in.local_uv.x), 0.0)) * (1.0 - max(sign(0.5-in.local_uv.y), 0.0));
    let mask = 1.0;
    
    let out = mix * mask;

    return out;
}
