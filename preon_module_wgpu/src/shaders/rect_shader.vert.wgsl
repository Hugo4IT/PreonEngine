[[block]]
struct Transform {
    transform: vec2<f32>;
};

struct VertexOutput {
    [[location(0)]] ratioOut: f32;
    [[location(1)]] radiusOut: vec4<f32>;
    [[location(2)]] colorOut: vec4<f32>;
    [[builtin(position)]] member: vec4<f32>;
};

var<private> position1: vec2<f32>;
var<private> tex_coords1: vec2<f32>;
var<private> radius1: vec4<f32>;
var<private> dimensions1: vec4<f32>;
var<private> color1: vec4<f32>;
var<private> ratioOut: f32;
var<private> radiusOut: vec4<f32>;
var<private> colorOut: vec4<f32>;
[[group(0), binding(0)]]
var<uniform> global: Transform;
var<private> gl_Position: vec4<f32>;

fn main1() {
    var rp: vec2<f32>;
    var rs: vec2<f32>;

    let e10: vec4<f32> = dimensions1;
    let e12: vec4<f32> = dimensions1;
    ratioOut = (e10.z / e12.w);
    let e15: vec4<f32> = radius1;
    radiusOut = e15;
    let e16: vec4<f32> = color1;
    colorOut = e16;
    let e17: vec4<f32> = dimensions1;
    let e19: vec4<f32> = dimensions1;
    rp = vec2<f32>(e17.x, e19.y);
    let e23: vec4<f32> = dimensions1;
    let e25: vec4<f32> = dimensions1;
    rs = vec2<f32>(e23.z, e25.w);
    let e34: vec2<f32> = global.transform;
    let e40: vec2<f32> = rp;
    let e43: vec2<f32> = global.transform;
    let e44: vec2<f32> = rs;
    let e46: vec2<f32> = position1;
    gl_Position = vec4<f32>(((vec2<f32>(-(1.0), 1.0) + ((e34 * vec2<f32>(1.0, -(1.0))) * e40)) + ((e43 * e44) * e46)), 0.0, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(1)]] tex_coords: vec2<f32>, [[location(2)]] radius: vec4<f32>, [[location(3)]] dimensions: vec4<f32>, [[location(4)]] color: vec4<f32>) -> VertexOutput {
    position1 = position;
    tex_coords1 = tex_coords;
    radius1 = radius;
    dimensions1 = dimensions;
    color1 = color;
    main1();
    let e29: f32 = ratioOut;
    let e31: vec4<f32> = radiusOut;
    let e33: vec4<f32> = colorOut;
    let e35: vec4<f32> = gl_Position;
    return VertexOutput(e29, e31, e33, e35);
}
