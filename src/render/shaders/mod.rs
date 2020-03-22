use wgpu::{Device, ShaderModule};

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule){
    let vs_src = include_str!("./shader.vert");
    let fs_src = include_str!("./shader.frag");

    let vs_spirv = glsl_to_spirv::compile(vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
    let fs_spirv = glsl_to_spirv::compile(fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();

    let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
    let fs_data = wgpu::read_spirv(fs_spirv).unwrap();

    let vs_module = device.create_shader_module(&vs_data);
    let fs_module = device.create_shader_module(&fs_data);

    (vs_module, fs_module)
}