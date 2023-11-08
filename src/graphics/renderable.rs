use wgpu::{BindGroupLayout, Queue, BufferAddress};
use wgpu::{util::DeviceExt, Device, SurfaceConfiguration, ShaderModule, VertexBufferLayout};

use super::{Vertex, InstanceVertex};

pub struct Renderable {
    pub pipeline: wgpu::RenderPipeline,
}

impl Renderable {
    fn new(
        device: &Device,
        config: &SurfaceConfiguration,
        shader: &ShaderModule,
        vertex_layouts: &[VertexBufferLayout<'_>],
        bind_layouts: &Vec<&BindGroupLayout>) -> Self {

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &bind_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: vertex_layouts,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0, // all masks
                alpha_to_coverage_enabled: false, // anti-aliasing
            },
            multiview: None,
        });

        Self {
            pipeline,
        }
    }
}

pub struct Index {
    pub renderable: Renderable,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Index {
    pub fn new(device: &Device,
            config: &SurfaceConfiguration,
            shader: &ShaderModule,
            vertex_layouts: &[VertexBufferLayout<'static>],
            vertices: &[Vertex],
            indices: &[u16],
            bind_layouts: &Vec<&BindGroupLayout>) -> Self {

        let renderable = Renderable::new(device, config, shader, vertex_layouts, bind_layouts);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;

        Self {
            renderable,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}

pub struct InstanceIndex {
    pub index: Index,
    pub instance_buffer: wgpu::Buffer,
    pub instance_data: Vec<InstanceVertex>,
}

impl InstanceIndex {
    pub fn new(device: &Device,
        config: &SurfaceConfiguration,
        shader: &ShaderModule,
        vertex_layouts: &[VertexBufferLayout<'static>],
        vertices: &[Vertex],
        indices: &[u16],
        instances: &[InstanceVertex],
        bind_layouts: &Vec<&BindGroupLayout>) -> Self {

        let index = Index::new(device, config, shader, vertex_layouts, vertices, indices, bind_layouts);
        
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            index,
            instance_buffer,
            instance_data: instances.to_vec(),
        }
    }

    pub fn add_instance(&mut self, device: &Device, data: InstanceVertex) {
        self.instance_data.push(data);
        
        self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&self.instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn update_instance_buffer(&self, queue: &Queue, offset: u32, data: InstanceVertex) {
        queue.write_buffer(
            &self.instance_buffer, 
            (offset * std::mem::size_of::<InstanceVertex>() as u32) as BufferAddress, 
            bytemuck::cast_slice(&[data]));
    }
}