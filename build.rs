use std::fs;
use std::process::Command;

fn main() {
    // UI
    println!("cargo:rerun-if-changed=assets/shaders/ui.frag");
    fs::remove_file("./assets/shaders/ui_frag.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/ui_frag.spv")
        .arg("./assets/shaders/ui.frag")
        .output()
        .expect("failed to execute process");

    println!("cargo:rerun-if-changed=assets/shaders/ui.vert");
    fs::remove_file("./assets/shaders/ui_vert.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/ui_vert.spv")
        .arg("./assets/shaders/ui.vert")
        .output()
        .expect("failed to execute process");

    // Loading

    println!("cargo:rerun-if-changed=assets/shaders/loading.frag");
    fs::remove_file("./assets/shaders/loading_frag.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/loading_frag.spv")
        .arg("./assets/shaders/loading.frag")
        .output()
        .expect("failed to execute process");

    println!("cargo:rerun-if-changed=assets/shaders/loading.vert");
    fs::remove_file("./assets/shaders/loading_vert.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/loading_vert.spv")
        .arg("./assets/shaders/loading.vert")
        .output()
        .expect("failed to execute process");

    // Main Shader

    println!("cargo:rerun-if-changed=assets/shaders/shader.frag");
    fs::remove_file("./assets/shaders/shader_frag.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/shader_frag.spv")
        .arg("./assets/shaders/shader.frag")
        .output()
        .expect("failed to execute process");

    println!("cargo:rerun-if-changed=assets/shaders/shader.vert");
    fs::remove_file("./assets/shaders/shader_vert.spv");
    Command::new("glslangValidator")
        .arg("-H")
        .arg("-V")
        .arg("-o")
        .arg("./assets/shaders/shader_vert.spv")
        .arg("./assets/shaders/shader.vert")
        .output()
        .expect("failed to execute process");
}
