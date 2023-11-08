use image::GenericImageView;
use wgpu::{Device, Queue, BindGroupLayout, BindGroup};

pub struct Texture2D {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture2D {
    pub fn new(device: &Device, queue: &Queue, file: &str) -> Self {
        let tex_bytes = std::fs::read(file)
            .expect("Cannot read texture image file");

        let tex_image = image::load_from_memory(&tex_bytes)
            .expect("Cannot load image from memory");

        let tex_rgba = tex_image.to_rgba8();

        let tex_dimension = tex_image.dimensions();

        let tex_size = wgpu::Extent3d {
            width: tex_dimension.0,
            height: tex_dimension.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: tex_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &tex_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * tex_dimension.0),
                rows_per_image: Some(tex_dimension.1),
            },
            tex_size
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn create_binding(&self, device: &Device, index: u32) -> (BindGroupLayout, BindGroup) {

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry { // texture entry
                        binding: index,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry { // sampler entry
                        binding: index + 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: index,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: index + 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        return (bind_layout, bind_group)
    }
}