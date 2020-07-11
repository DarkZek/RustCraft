use wgpu::{Device, ShaderModule};

pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs_src = include_bytes!("../../../assets/shaders/shader_vert.spv");
    let fs_src = include_bytes!("../../../assets/shaders/shader_frag.spv");

    let vs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        bytes_to_shader(vs_src).as_slice(),
    ));
    let fs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
        bytes_to_shader(fs_src).as_slice(),
    ));

    (vs_module, fs_module)
}

pub fn bytes_to_shader(bytes: &[u8]) -> Vec<u32> {
    let mut out = Vec::new();

    for i in 0..(bytes.len() / 4) {
        let range = i * 4;

        let n = u32::from_le_bytes([
            bytes[range + 0],
            bytes[range + 1],
            bytes[range + 2],
            bytes[range + 3],
        ]);
        out.push(n);
    }

    out
}
