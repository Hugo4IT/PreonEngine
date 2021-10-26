struct FragmentOutput {
    [[location(0)]] FragColor: vec4<f32>;
};

var<private> ratio1: f32;
var<private> radius1: vec4<f32>;
var<private> color1: vec4<f32>;
var<private> FragColor: vec4<f32>;

fn main1() {
    let e4: vec4<f32> = color1;
    FragColor = e4;
    return;
}

[[stage(fragment)]]
fn main([[location(0)]] ratio: f32, [[location(1)]] radius: vec4<f32>, [[location(2)]] color: vec4<f32>) -> FragmentOutput {
    ratio1 = ratio;
    radius1 = radius;
    color1 = color;
    main1();
    let e15: vec4<f32> = FragColor;
    return FragmentOutput(e15);
}
