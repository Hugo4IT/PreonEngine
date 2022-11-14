use preon_engine::{
    components::label::LabelConfig,
    rendering::{PreonFont, PreonShape},
    types::PreonVector,
};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Layout, Section, Text};

pub struct TextShape {
    brushes: Vec<Option<GlyphBrush<()>>>,
    staging_belt: wgpu::util::StagingBelt,
}

impl TextShape {
    pub fn new(
        device: &wgpu::Device,
        fonts: &'static [&'static PreonFont],
        format: wgpu::TextureFormat,
    ) -> Self {
        let mut brushes = Vec::new();

        for font in fonts.iter() {
            macro_rules! add_font {
                ($($target:ident),*) => {
                    $(
                    if let Some(data) = font.$target {
                        brushes.push(Some(
                            GlyphBrushBuilder::using_font(ab_glyph::FontArc::try_from_slice(data).unwrap())
                                .build(device, format),
                        ));
                    }
                    )*
                }
            }

            add_font!(
                w100, w100i, w200, w200i, w300, w300i, w400, w400i, w500, w500i, w600, w600i, w700,
                w700i, w800, w800i, w900, w900i
            );
        }

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            brushes,
            staging_belt,
        }
    }

    pub fn build(&mut self, shape: PreonShape, z_index: f32) {
        if let PreonShape::Text {
            text_settings,
            position,
            size,
            ref text,
        } = shape
        {
            let settings = LabelConfig::decode(text_settings);
            let brush = self
                .brushes
                .get_mut(settings.font_index as usize)
                .unwrap()
                .as_mut()
                .unwrap();

            let (r, g, b, a) = settings.color.into_f32_tuple();

            brush.queue(Section {
                screen_position: (position.x as f32, position.y as f32),
                bounds: (size.x as f32, size.y as f32),
                layout: Layout::default_wrap(),
                text: vec![Text::new(text)
                    .with_color([r, g, b, a])
                    .with_scale(settings.size as f32)
                    .with_z(z_index)],
            });
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        screen_size: PreonVector<i32>,
    ) {
        for brush in self.brushes.iter_mut() {
            brush
                .as_mut()
                .unwrap()
                .draw_queued(
                    device,
                    &mut self.staging_belt,
                    encoder,
                    view,
                    screen_size.x as u32,
                    screen_size.y as u32,
                )
                .unwrap();
        }

        self.staging_belt.finish();
    }
}
