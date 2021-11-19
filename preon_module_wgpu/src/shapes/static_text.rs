// use super::static_texture::StaticTextureShape;

// pub struct StaticTextShape {
//     pub texture: StaticTextureShape,
// }

// impl StaticTextShape {
//     pub fn new(
//         device: &wgpu::Device,
//         config: &wgpu::SurfaceConfiguration,
//         queue: &wgpu::Queue,
//         transform_bind_group_layout: &wgpu::BindGroupLayout,
//         strings: &[&'static str],
//     ) -> Self {
//         let target_font = fontdue::Font::from_bytes(
//             &include_bytes!("../../../res/Montserrat-Regular.ttf"),
//             fontdue::FontSettings {
//                 collection_index: 0,
//                 scale: 16.0,
//             },
//         );

//         for string in strings.iter() {}

//         let renderer_glyphs =
//             StaticTextureShape::new(device, config, queue, transform_bind_group_layout, &[]);

//         Self {}
//     }
// }
