use crate::services::asset_service::{AssetService, ResourcePack};
use std::fs;
use zip::ZipArchive;
use std::fs::File;
use image::{ImageFormat, DynamicImage};
use std::collections::HashMap;
use std::io::Read;
use std::time::Instant;

impl AssetService {
    pub(crate) fn get_resource_packs(path: &str) -> Vec<String> {
        // Load a list of resource packs
        match fs::read_dir(path) {
            Ok(files) => {
                let mut packs = Vec::new();
                for file in files {
                    if let Ok(file) = file {
                        if file.file_type().unwrap().is_file() &&
                            file.file_name().to_str().unwrap().ends_with("zip") {
                            packs.push(file.file_name().to_str().unwrap().to_string());
                        }
                    }
                }
                packs
            }
            Err(e) => {
                println!("Failed to load resource packs directory: {}", e);
                Vec::new()
            }
        }
    }

    pub(crate) fn load_resource_pack(path: &str) -> ResourcePack {
        let start_time = Instant::now();
        let zipfile = std::fs::File::open(&path).unwrap();

        let mut archive = zip::ZipArchive::new(zipfile).unwrap();

        let textures = load_resources(&mut archive);

        log!(format!("Took {} seconds to load texture pack {}", Instant::now().duration_since(start_time).as_secs_f32(), path));

        ResourcePack {
            name: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            textures
        }
    }
}

fn load_resources(archive: &mut ZipArchive<File>) -> HashMap<String, DynamicImage> {
    let mut out = HashMap::new();

    for i in 0..archive.len() {
        let mut item = archive.by_index(i).unwrap();

        if item.is_file() && item.name().ends_with(".png") {
            let mut data: Vec<u8> = Vec::new();
            if let Err(e) = item.read_to_end(&mut data) {
                println!("Error reading resource {} - {}", item.name(), e);
                continue;
            }

            match image::load_from_memory_with_format(data.as_slice(), ImageFormat::Png) {
                Ok(img) => {
                    out.insert(format_file_name(item.name()), img);
                },
                Err(e) => {
                    println!("Error reading resource {} - {}", item.name(), e);
                }
            };
        }
    }

    out
}

fn format_file_name(name: &str) -> String {
    // Remove the first three directories, usually RESOURCE_PACK_NAME/assets/minecraft/
    let mut slash_count = 0;
    let mut out = String::new();
    if name.contains("pack.png") { return String::from("pack.png"); }

    for char in name.chars() {
        if slash_count == 3 {
            out.push(char);
        } else {
            if char == '/' || char == '\\' {
                slash_count += 1;
            }
        }
    }

    // Remove .png
    out.truncate(out.len() - 4);

    out
}