struct FragmentOutput {
    [[location(0)]] FragColor: vec4<f32>;
};

var<private> uv1: vec2<f32>;
[[group(1), binding(0)]]
var inTexture: texture_2d<f32>;
[[group(1), binding(1)]]
var inSampler: sampler;
var<private> FragColor: vec4<f32>;

fn main1() {
    let e5: vec2<f32> = uv1;
    let e6: vec4<f32> = textureSample(inTexture, inSampler, e5);
    FragColor = e6;
    return;
}

[[stage(fragment)]]
fn main([[location(0)]] uv: vec2<f32>) -> FragmentOutput {
    uv1 = uv;
    main1();
    let e11: vec4<f32> = FragColor;
    return FragmentOutput(e11);
}
