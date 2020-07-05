use wgpu::{Device, ShaderModule, ShaderModuleSource};

/// Load shaders using shaderc
pub fn load_shaders(device: &Device) -> (ShaderModule, ShaderModule) {
    let vs_src = include_str!("../../../assets/shaders/shader.vert");
    let fs_src = include_str!("../../../assets/shaders/shader.frag");

    let mut compiler = shaderc::Compiler::new().unwrap();

    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("EP", Some("main"));

    let vs_spirv = compiler
        .compile_into_spirv(
            vs_src,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            Some(&options),
        )
        .unwrap();

    let fs_spirv = compiler
        .compile_into_spirv(
            fs_src,
            shaderc::ShaderKind::Fragment,
            "shader.frag",
            "main",
            Some(&options),
        )
        .unwrap();

    let vs_module = device.create_shader_module(ShaderModuleSource::SpirV(vs_spirv.as_binary()));
    let fs_module = device.create_shader_module(ShaderModuleSource::SpirV(fs_spirv.as_binary()));

    (vs_module, fs_module)
}
