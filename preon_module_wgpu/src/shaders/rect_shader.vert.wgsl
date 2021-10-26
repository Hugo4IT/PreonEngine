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

    let e9: vec4<f32> = dimensions1;
    let e11: vec4<f32> = dimensions1;
    ratioOut = (e9.z / e11.w);
    let e14: vec4<f32> = radius1;
    radiusOut = e14;
    let e15: vec4<f32> = color1;
    colorOut = e15;
    let e16: vec4<f32> = dimensions1;
    let e18: vec4<f32> = dimensions1;
    rp = vec2<f32>(e16.x, e18.y);
    let e22: vec4<f32> = dimensions1;
    let e24: vec4<f32> = dimensions1;
    rs = vec2<f32>(e22.z, e24.w);
    let e33: vec2<f32> = global.transform;
    let e39: vec2<f32> = rp;
    let e42: vec2<f32> = global.transform;
    let e43: vec2<f32> = rs;
    let e45: vec2<f32> = position1;
    gl_Position = vec4<f32>(((vec2<f32>(-(1.0), 1.0) + ((e33 * vec2<f32>(1.0, -(1.0))) * e39)) + ((e42 * e43) * e45)), 0.0, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(1)]] radius: vec4<f32>, [[location(2)]] dimensions: vec4<f32>, [[location(3)]] color: vec4<f32>) -> VertexOutput {
    position1 = position;
    radius1 = radius;
    dimensions1 = dimensions;
    color1 = color;
    main1();
    let e25: f32 = ratioOut;
    let e27: vec4<f32> = radiusOut;
    let e29: vec4<f32> = colorOut;
    let e31: vec4<f32> = gl_Position;
    return VertexOutput(e25, e27, e29, e31);
}
