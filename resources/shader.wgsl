struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct InstanceVertexIn {
    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct CameraUniform {
    view_projection: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    in: VertexIn,
    instance: InstanceVertexIn
) -> VertexOutput {
    let model = mat4x4<f32> (
        instance.model_0,
        instance.model_1,
        instance.model_2,
        instance.model_3,
    );

    var out: VertexOutput;
    out.color = vec4<f32>(in.color, 1.0);
    out.position = camera.view_projection * model * vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}