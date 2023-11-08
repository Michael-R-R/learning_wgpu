use image::{GenericImageView, RgbaImage};
use wgpu::{Device, Texture, Queue, TextureView, Sampler, BindGroupLayout, BindGroup};

pub struct Texture2D {
    pub bind_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl Texture2D {
    pub fn new(device: &Device, queue: &Queue, file: &str) -> Self {

        let (rgba,
            dimension,
            size, 
            texture) = Texture2D::create(device, file);

        Texture2D::write(queue, &rgba, dimension, size, &texture);

        let (view, sampler) = Texture2D::create_sampler(device, &texture);

        let bind_layout = Texture2D::create_bind_layout(device);

        let bind_group = Texture2D::create_bind_group(device, &view, &sampler, &bind_layout); 

        Self {
            bind_layout,
            bind_group,
        }
    }

    fn create(device: &Device, file: &str) -> (RgbaImage, (u32, u32), wgpu::Extent3d, Texture) {
        let tex_bytes = std::fs::read(file)
            .expect("Cannot read diffuse texture file");
        let tex_image = image::load_from_memory(&tex_bytes).unwrap();
        let tex_rgba = tex_image.to_rgba8();

        let tex_dimension = tex_image.dimensions();
        let tex_size = wgpu::Extent3d {
            width: tex_dimension.0,
            height: tex_dimension.1,
            depth_or_array_layers: 1,
        };
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Diffuse texture"),
            size: tex_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        (tex_rgba, tex_dimension, tex_size, tex)
    }

    fn write(queue: &Queue, rgba: &RgbaImage, dimension: (u32,u32), size: wgpu::Extent3d, texture: &Texture) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimension.0),
                rows_per_image: Some(dimension.1),
            },
            size
        );
    }

    fn create_sampler(device: &Device, texture: &Texture) -> (TextureView, Sampler) {
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

        (view, sampler)
    }

    fn create_bind_layout(device: &Device) -> BindGroupLayout {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        return layout
    }

    fn create_bind_group(device: &Device, view: &TextureView, sampler: &Sampler, layout: &BindGroupLayout) -> BindGroup {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        return bind_group;
    }

}