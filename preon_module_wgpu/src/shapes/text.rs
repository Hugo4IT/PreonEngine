use preon_engine::{rendering::PreonShape, types::PreonVector};
use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Layout, Section, Text};

pub struct TextShape {
    brushes: Vec<Option<GlyphBrush<()>>>,
    staging_belt: wgpu::util::StagingBelt,
}

impl TextShape {
    pub fn new(
        device: &wgpu::Device,
        fonts: &'static [&'static [u8]],
        format: wgpu::TextureFormat,
    ) -> Self {
        let mut brushes = Vec::new();

        for font in fonts.iter() {
            brushes.push(Some(
                GlyphBrushBuilder::using_font(ab_glyph::FontArc::try_from_slice(*font).unwrap())
                    .build(device, format),
            ));
        }

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            brushes,
            staging_belt,
        }
    }

    pub fn build(&mut self, shape: PreonShape, z_index: f32) {
        if let PreonShape::Text {
            font_index,
            position,
            size,
            ref text,
        } = shape
        {
            let brush = self.brushes.get_mut(font_index).unwrap().as_mut().unwrap();

            brush.queue(Section {
                screen_position: (position.x as f32, position.y as f32),
                bounds: (size.x as f32, size.y as f32),
                layout: Layout::default_wrap(),
                text: vec![Text::new(text)
                    .with_color([0.0, 0.0, 0.0, 1.0])
                    .with_scale(32.0)
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
