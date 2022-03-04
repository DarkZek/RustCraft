use crate::services::asset_service::{AssetService, ResourcePack};
use fnv::{FnvBuildHasher, FnvHashMap};
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Instant, SystemTime};
use zip::result::ZipResult;
use zip::ZipArchive;

impl AssetService {
    pub fn get_resource_packs(path: PathBuf) -> Vec<String> {
        // Load a list of resource packs
        match fs::read_dir(path) {
            Ok(files) => {
                let mut packs = Vec::new();
                for file in files {
                    if let Ok(file) = file {
                        if file.file_type().unwrap().is_file()
                            && file.file_name().to_str().unwrap().ends_with("zip")
                        {
                            packs.push(file.file_name().to_str().unwrap().to_string());
                        }
                    }
                }
                packs
            }
            Err(e) => {
                log_error!("Failed to load resource packs directory: {}", e);
                Vec::new()
            }
        }
    }

    pub fn load_resource_pack(path: PathBuf) -> ZipResult<ResourcePack> {
        let start_time = Instant::now();
        let zipfile = std::fs::File::open(&path).unwrap();
        let metadata = fs::metadata(&path).unwrap();

        let mut archive = zip::ZipArchive::new(zipfile)?;

        let textures = load_resources(&mut archive);

        let name = path.file_name().unwrap().to_str().unwrap();

        log!(
            "Took {} seconds to load texture pack {}",
            Instant::now().duration_since(start_time).as_secs_f32(),
            name
        );

        Ok(ResourcePack {
            name: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            textures,
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
        })
    }
}

fn load_resources(archive: &mut ZipArchive<File>) -> HashMap<String, DynamicImage, FnvBuildHasher> {
    let mut out = FnvHashMap::default();

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
                }
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

    if name.contains("pack.png") {
        return String::from("pack.png");
    }

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
