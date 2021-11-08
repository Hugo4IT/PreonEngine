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
var<private> dimensions1: vec4<f32>;
var<private> uv_dimensions1: vec4<f32>;
var<private> uv: vec2<f32>;
[[group(0), binding(0)]]
var<uniform> global: Transform;
var<private> gl_Position: vec4<f32>;

fn main1() {
    var rp: vec2<f32>;
    var rs: vec2<f32>;

    let e7: vec4<f32> = uv_dimensions1;
    let e9: vec4<f32> = uv_dimensions1;
    let e11: vec4<f32> = uv_dimensions1;
    let e14: vec2<f32> = tex_coords1;
    let e16: vec4<f32> = uv_dimensions1;
    let e18: vec4<f32> = uv_dimensions1;
    let e20: vec4<f32> = uv_dimensions1;
    let e23: vec2<f32> = tex_coords1;
    let e26: vec4<f32> = uv_dimensions1;
    let e28: vec4<f32> = uv_dimensions1;
    let e30: vec4<f32> = uv_dimensions1;
    let e33: vec2<f32> = tex_coords1;
    let e35: vec4<f32> = uv_dimensions1;
    let e37: vec4<f32> = uv_dimensions1;
    let e39: vec4<f32> = uv_dimensions1;
    let e42: vec2<f32> = tex_coords1;
    uv = vec2<f32>(mix(e16.x, (e18.x + e20.z), e23.x), mix(e35.y, (e37.y + e39.w), e42.y));
    let e46: vec4<f32> = dimensions1;
    let e48: vec4<f32> = dimensions1;
    rp = vec2<f32>(e46.x, e48.y);
    let e52: vec4<f32> = dimensions1;
    let e54: vec4<f32> = dimensions1;
    rs = vec2<f32>(e52.z, e54.w);
    let e63: vec2<f32> = global.transform;
    let e69: vec2<f32> = rp;
    let e72: vec2<f32> = global.transform;
    let e73: vec2<f32> = rs;
    let e75: vec2<f32> = position1;
    gl_Position = vec4<f32>(((vec2<f32>(-(1.0), 1.0) + ((e63 * vec2<f32>(1.0, -(1.0))) * e69)) + ((e72 * e73) * e75)), 0.0, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(1)]] tex_coords: vec2<f32>, [[location(2)]] dimensions: vec4<f32>, [[location(3)]] uv_dimensions: vec4<f32>) -> VertexOutput {
    position1 = position;
    tex_coords1 = tex_coords;
    dimensions1 = dimensions;
    uv_dimensions1 = uv_dimensions;
    main1();
    let e21: vec2<f32> = uv;
    let e23: vec4<f32> = gl_Position;
    return VertexOutput(e21, e23);
}
