use wgpu::{ShaderModule, Device};

pub struct Shader {
    module: ShaderModule
}

impl Shader {
    pub fn new(file: &str, device: &Device) -> Self {
        
        let content = std::fs::read_to_string(file)
            .expect("Failed to read shader file");

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(content.into()),
        });

        Self {
            module,
        }
    }

    pub fn module(&self) -> &ShaderModule {
        &self.module
    }
}