use winit::event::Event;
use winit::{window::Window, event::WindowEvent};

use super::shapes;
use super::Camera;
use super::Vertex;
use super::Shader;
use super::InstanceIndex;
use super::InstanceVertex;
use super::GUI;

pub struct State {
    pub window: Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub gui: GUI,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let gui = GUI::new(&window, &device, &queue, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            gui,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn handle_event(&mut self, event: &Event<'_, ()>) {
        self.gui.handle_event(&self.window, event);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::ModifiersChanged(..) => { return false },
            WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(key) => {
                        match key {
                            // --- Match all input here --- //
                            _ => return false
                        }
                    },
                    None => return false
                }
            },
            _ => return false
        }
    }

    pub fn update(&mut self, _dt: f32) {
        
    }

    pub fn render(&mut self, dt: f32) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let i1 = InstanceVertex::new(cgmath::Matrix4::from_translation(cgmath::Vector3 { x: -150.0, y: 0.0, z: 0.0 }) * cgmath::Matrix4::from_scale(100.0));
        let i2 = InstanceVertex::new(cgmath::Matrix4::from_translation(cgmath::Vector3 { x: 150.0, y: 0.0, z: 0.0 }) * cgmath::Matrix4::from_scale(100.0));
        
        let mut c = Camera::new(&self.device, &self.config, -1000.0, 1000.0);
        let s = Shader::new("resources\\shader.wgsl", &self.device);
        let r = InstanceIndex::new(
            &self.device,
            &self.config, 
            &s.module(), 
            &vec![Vertex::layout(), InstanceVertex::layout()],
            &shapes::plane(),
            &shapes::plane_indices(),
            &vec![i1, i2],
            &vec![&c.bind_layout]);

        c.update_buffer(&self.queue);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2, g: 0.2, b: 0.2, a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&r.index.renderable.pipeline);
            render_pass.set_bind_group(0, &c.bind_group, &[]);
            render_pass.set_vertex_buffer(0, r.index.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, r.instance_buffer.slice(..));
            render_pass.set_index_buffer(r.index.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..r.index.num_indices, 0, 0..r.num_instances);

            self.gui.render(dt, 
                &self.window, 
                &self.device, 
                &self.queue, 
                &mut render_pass, 
                [self.config.width as f32, self.config.height as f32]);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }

}
