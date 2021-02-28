use std::borrow::Cow;
use wgpu::{Device, ShaderModule};

pub fn load_shaders(device: &Device, shader_bytes: (&[u8], &[u8])) -> (ShaderModule, ShaderModule) {
    let vs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(Cow::Borrowed(
        bytes_to_shader(shader_bytes.0).as_slice(),
    )));
    let fs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(Cow::Borrowed(
        bytes_to_shader(shader_bytes.1).as_slice(),
    )));

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
