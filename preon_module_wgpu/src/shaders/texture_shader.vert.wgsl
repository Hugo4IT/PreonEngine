[[block]]
struct Transform {
    transform: vec2<f32>;
};

struct VertexOutput {
    [[location(0)]] uv: vec2<f32>;
    [[builtin(position)]] member: vec4<f32>;
};

var<private> position1: vec2<f32>;
var<private> tex_coords1: vec2<f32>;
var<private> z_index1: f32;
var<private> dimensions1: vec4<f32>;
var<private> uv_dimensions1: vec4<f32>;
var<private> uv: vec2<f32>;
[[group(0), binding(0)]]
var<uniform> global: Transform;
var<private> gl_Position: vec4<f32>;

fn main1() {
    var rp: vec2<f32>;
    var rs: vec2<f32>;

    let e8: vec4<f32> = uv_dimensions1;
    let e10: vec4<f32> = uv_dimensions1;
    let e12: vec4<f32> = uv_dimensions1;
    let e15: vec2<f32> = tex_coords1;
    let e17: vec4<f32> = uv_dimensions1;
    let e19: vec4<f32> = uv_dimensions1;
    let e21: vec4<f32> = uv_dimensions1;
    let e24: vec2<f32> = tex_coords1;
    let e27: vec4<f32> = uv_dimensions1;
    let e29: vec4<f32> = uv_dimensions1;
    let e31: vec4<f32> = uv_dimensions1;
    let e34: vec2<f32> = tex_coords1;
    let e36: vec4<f32> = uv_dimensions1;
    let e38: vec4<f32> = uv_dimensions1;
    let e40: vec4<f32> = uv_dimensions1;
    let e43: vec2<f32> = tex_coords1;
    uv = vec2<f32>(mix(e17.x, (e19.x + e21.z), e24.x), mix(e36.y, (e38.y + e40.w), e43.y));
    let e47: vec4<f32> = dimensions1;
    let e49: vec4<f32> = dimensions1;
    rp = vec2<f32>(e47.x, e49.y);
    let e53: vec4<f32> = dimensions1;
    let e55: vec4<f32> = dimensions1;
    rs = vec2<f32>(e53.z, e55.w);
    let e64: vec2<f32> = global.transform;
    let e70: vec2<f32> = rp;
    let e73: vec2<f32> = global.transform;
    let e74: vec2<f32> = rs;
    let e76: vec2<f32> = position1;
    let e79: f32 = z_index1;
    gl_Position = vec4<f32>(((vec2<f32>(-(1.0), 1.0) + ((e64 * vec2<f32>(1.0, -(1.0))) * e70)) + ((e73 * e74) * e76)), e79, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(1)]] tex_coords: vec2<f32>, [[location(2)]] z_index: f32, [[location(3)]] dimensions: vec4<f32>, [[location(4)]] uv_dimensions: vec4<f32>) -> VertexOutput {
    position1 = position;
    tex_coords1 = tex_coords;
    z_index1 = z_index;
    dimensions1 = dimensions;
    uv_dimensions1 = uv_dimensions;
    main1();
    let e25: vec2<f32> = uv;
    let e27: vec4<f32> = gl_Position;
    return VertexOutput(e25, e27);
}
