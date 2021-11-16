pub struct Texture {
    pub raw: Option<Vec<u8>>,
    pub wgpu: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn from_image(bytes: &[u8], device: &wgpu::Device, store_bytes: bool) -> Texture {
        let image = image::load_from_memory(bytes).unwrap();

        use image::GenericImageView;
        let dimensions = image.dimensions();

        Self::from_bytes(
            image
                .as_rgba8()
                .unwrap()
                .pixels()
                .flat_map(|p| p.0.iter().map(|b| *b))
                .collect::<Vec<u8>>()
                .as_slice(),
            wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            },
            device,
            store_bytes,
        )
    }

    pub fn from_bytes(
        bytes: &[u8],
        dimensions: wgpu::Extent3d,
        device: &wgpu::Device,
        store_bytes: bool,
    ) -> Texture {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("A texture"),
            size: dimensions,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            raw: if store_bytes {
                Some(bytes.to_vec())
            } else {
                None
            },
            wgpu: texture,
            view,
            sampler,
        }
    }

    pub fn new_depth(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Texture {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            label: Some("Depth Sampler"),
            ..Default::default()
        });

        Self {
            raw: None,
            wgpu: texture,
            view,
            sampler,
        }
    }
}

pub struct TextureSheet {
    pub texture: Texture,
    pub indices: Vec<[f32; 4]>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl TextureSheet {
    pub fn from_images(buffers: &[&[u8]], device: &wgpu::Device, queue: &wgpu::Queue) -> TextureSheet {
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("BindGroupLayout for textures"),
            });

        let mut textures: Vec<sheep::InputSprite> = Vec::new();
        for buffer in buffers.iter() {
            let image = image::load_from_memory(*buffer).unwrap();

            use image::GenericImageView;
            let dimensions = image.dimensions();

            textures.push(sheep::InputSprite {
                bytes: image
                    .as_rgba8()
                    .unwrap()
                    .pixels()
                    .flat_map(|p| p.0.iter().map(|b| *b))
                    .collect::<Vec<u8>>(),
                dimensions,
            });
        }

        let packed = sheep::pack::<sheep::MaxrectsPacker>(textures, 4, Default::default());
        let packed = packed.into_iter().next().unwrap();
        let size = wgpu::Extent3d {
            width: packed.dimensions.0,
            height: packed.dimensions.1,
            depth_or_array_layers: 1,
        };

        let indices = sheep::encode::<TextureSheetEncoder>(&packed, 0);

        let texture = Texture::from_bytes(packed.bytes.as_slice(), size, device, false);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture.wgpu,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            packed.bytes.as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * packed.dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(packed.dimensions.1),
            },
            size,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("BindGroup for StaticTexture atlas"),
        });

        Self { texture, indices, bind_group_layout, bind_group }
    }
}

pub struct TextureSheetEncoder {}
impl sheep::Format for TextureSheetEncoder {
    type Data = Vec<[f32; 4]>;
    type Options = u8;

    fn encode(
        dimensions: (u32, u32),
        sprites: &[sheep::SpriteAnchor],
        _options: Self::Options,
    ) -> Self::Data {
        let mut _sprites = sprites.to_vec();
        _sprites.sort_by(|left, right| left.id.cmp(&right.id));

        let mut data = Vec::with_capacity(_sprites.len());
        _sprites.iter().for_each(|anchor| {
            data.push([
                (anchor.position.0 as f32) / (dimensions.0 as f32),
                (anchor.position.1 as f32) / (dimensions.1 as f32),
                (anchor.dimensions.0 as f32) / (dimensions.0 as f32),
                (anchor.dimensions.1 as f32) / (dimensions.1 as f32),
            ])
        });
        data
    }
}
