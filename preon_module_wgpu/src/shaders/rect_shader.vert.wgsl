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
var<private> z_index1: f32;
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

    let e11: vec4<f32> = dimensions1;
    let e13: vec4<f32> = dimensions1;
    ratioOut = (e11.z / e13.w);
    let e16: vec4<f32> = radius1;
    radiusOut = e16;
    let e17: vec4<f32> = color1;
    colorOut = e17;
    let e18: vec4<f32> = dimensions1;
    let e20: vec4<f32> = dimensions1;
    rp = vec2<f32>(e18.x, e20.y);
    let e24: vec4<f32> = dimensions1;
    let e26: vec4<f32> = dimensions1;
    rs = vec2<f32>(e24.z, e26.w);
    let e35: vec2<f32> = global.transform;
    let e41: vec2<f32> = rp;
    let e44: vec2<f32> = global.transform;
    let e45: vec2<f32> = rs;
    let e47: vec2<f32> = position1;
    let e50: f32 = z_index1;
    gl_Position = vec4<f32>(((vec2<f32>(-(1.0), 1.0) + ((e35 * vec2<f32>(1.0, -(1.0))) * e41)) + ((e44 * e45) * e47)), e50, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(1)]] tex_coords: vec2<f32>, [[location(2)]] z_index: f32, [[location(3)]] radius: vec4<f32>, [[location(4)]] dimensions: vec4<f32>, [[location(5)]] color: vec4<f32>) -> VertexOutput {
    position1 = position;
    tex_coords1 = tex_coords;
    z_index1 = z_index;
    radius1 = radius;
    dimensions1 = dimensions;
    color1 = color;
    main1();
    let e33: f32 = ratioOut;
    let e35: vec4<f32> = radiusOut;
    let e37: vec4<f32> = colorOut;
    let e39: vec4<f32> = gl_Position;
    return VertexOutput(e33, e35, e37, e39);
}
