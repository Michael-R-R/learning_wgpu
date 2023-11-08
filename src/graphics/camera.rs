use cgmath::{Point3, Vector3, SquareMatrix};
use wgpu::{Device, util::DeviceExt, Buffer, BindGroupLayout, BindGroup, SurfaceConfiguration, Queue};

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    width: f32,
    height: f32,
    znear: f32,
    zfar: f32,
    pub buffer: Buffer,
    pub bind_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

impl Camera {
    pub fn new(device: &Device,
         config: &SurfaceConfiguration, 
         buffer: Buffer, 
         near: f32, 
         far: f32) -> Self {

        let bind_layout = Camera::create_layout(device);
        let bind_group = Camera::create_bind_group(device, &bind_layout, &buffer);

        Self 
        { 
            eye: Point3::new(0.0, 0.0, -1.0),
            target: Point3::new(0.0, 0.0, 0.0), 
            up: Vector3::unit_y(), 
            width: config.width as f32,
            height: config.width as f32, 
            znear: near, 
            zfar: far, 
            buffer,
            bind_layout,
            bind_group,
        }
    }

    fn create_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        })
    }

    fn create_bind_group(device: &Device, layout: &BindGroupLayout, buffer: &Buffer) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        })
    }

    pub fn view_projection(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::ortho(self.width / 2.0, 
            -self.width / 2.0, 
            -self.height / 2.0, 
            self.height / 2.0, 
            self.znear, 
            self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformBuffer {
    view_projection: [[f32; 4]; 4],
}

impl UniformBuffer {
    pub fn new(device: &Device) -> (Self, Buffer) {
        let u_buffer = UniformBuffer { 
            view_projection: cgmath::Matrix4::identity().into() 
        };

        let buffer = u_buffer.init_buffer(device);

        (u_buffer, buffer)
    }

    fn init_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn update_buffer(&mut self, queue: &Queue, camera: &Camera) {
        self.view_projection = camera.view_projection().into();
        queue.write_buffer(&camera.buffer, 0, bytemuck::cast_slice(&[*self]));
    }
}
