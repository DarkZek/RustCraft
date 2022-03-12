use std::fs;
use std::fs::File;
use std::process::Command;

fn main() {
    if Command::new("glslangValidator").output().is_err() {
        println!("Warning: GLSL Validator not installed, shaders will not be updated. Please install Vulkan SDK and add to path");
        return;
    }

    // Shaders
    let shaders = [
        "./shaders/ui_text",
        "./shaders/ui_images",
        "./shaders/loading",
        "./shaders/shader",
        "./shaders/background",
        "./shaders/gaussian",
        "./shaders/addition",
        "./shaders/outline",
        "./shaders/ssao",
        "./shaders/multiply",
        "./rc_ui/shaders/default",
        "./rc_ui/shaders/combine",
    ];

    for shader in shaders.iter() {
        assert_eq!(true, File::open(format!("{}.frag", shader)).is_ok());
        assert_eq!(true, File::open(format!("{}.vert", shader)).is_ok());

        println!("cargo:rerun-if-changed={}.frag", shader);
        fs::remove_file(format!("{}_frag.spv", shader));
        Command::new("glslangValidator")
            .arg("-H")
            .arg("-V")
            .arg("-o")
            .arg(format!("{}_frag.spv", shader))
            .arg(format!("{}.frag", shader))
            .output()
            .expect("failed to execute process");

        println!("cargo:rerun-if-changed={}.vert", shader);
        fs::remove_file(format!("{}_vert.spv", shader));
        Command::new("glslangValidator")
            .arg("-H")
            .arg("-V")
            .arg("-o")
            .arg(format!("{}_vert.spv", shader))
            .arg(format!("{}.vert", shader))
            .output()
            .expect("failed to execute process");
    }
}
