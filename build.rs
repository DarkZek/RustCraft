use std::fs;
use std::process::Command;

fn main() {
    // Shaders
    let shaders = ["ui_text", "ui_images", "loading", "shader"];

    for shader in shaders.iter() {
        println!("cargo:rerun-if-changed=assets/shaders/{}.frag", shader);
        fs::remove_file(format!("./assets/shaders/{}_frag.spv", shader));
        Command::new("glslangValidator")
            .arg("-H")
            .arg("-V")
            .arg("-o")
            .arg(format!("./assets/shaders/{}_frag.spv", shader))
            .arg(format!("./assets/shaders/{}.frag", shader))
            .output()
            .expect("failed to execute process");

        println!("cargo:rerun-if-changed=assets/shaders/{}.vert", shader);
        fs::remove_file(format!("./assets/shaders/{}_vert.spv", shader));
        Command::new("glslangValidator")
            .arg("-H")
            .arg("-V")
            .arg("-o")
            .arg(format!("./assets/shaders/{}_vert.spv", shader))
            .arg(format!("./assets/shaders/{}.vert", shader))
            .output()
            .expect("failed to execute process");
    }
}
