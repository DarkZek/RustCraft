use app_dirs::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

pub mod key_mappings;

pub const CHUNK_SIZE: usize = 16;
pub const APP_INFO: AppInfo = AppInfo {
    name: "RustCraft",
    author: "DarkZek",
};

//TODO: Make it actually load from a text file

/// Responsible for loading settings from environment variables, loading settings file and saving the settings file. Giving us persistent storage for variables.
/// This is separate from the loading of world files, it only operates in text.
///
/// ## Loading
///
/// Loads settings file from only the local directory the executable is ran from.
///

pub struct SettingsService {
    pub path: PathBuf,
    pub settings_path: PathBuf,
    pub atlas_cache_reading: bool,
    pub atlas_cache_writing: bool,
    /// Changes the texture atlas to generate random textures instead
    pub debug_vertices: bool,
    pub debug_atlas: bool,
    pub backface_culling: bool,
    pub chunk_edge_faces: bool,
    pub debug_states: bool,
    pub config: SettingsFile,
}

impl Default for SettingsService {
    fn default() -> Self {
        unimplemented!()
    }
}

impl SettingsService {
    /// Creates a new instance of settings, loading the variables from the environment variables as well as settings file
    ///
    /// # Returns
    ///
    /// A new initialized `SettingsService`
    ///
    pub fn new() -> SettingsService {
        // Load resources directory
        let path = get_app_root(AppDataType::UserConfig, &APP_INFO).unwrap();

        // Create directories
        fs::create_dir_all(path.as_path()).unwrap();

        let mut settings_path = path.clone();
        settings_path.push("settings");
        settings_path.set_extension("json");

        // Check if settings.json exists, if it doesn't create it with defaults
        if !settings_path.exists() {
            match File::create(&settings_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write(
                        serde_json::to_string_pretty(&SettingsFile::default())
                            .unwrap()
                            .as_bytes(),
                    ) {
                        log_error!("Failed to write {:?}: {}", settings_path, e);
                        std::process::exit(0x5);
                    } else {
                        log!("Created {:?}settings.json", settings_path);
                    }
                }
                Err(e) => {
                    log_error!("Failed to write {:?}: {}", settings_path, e);
                    std::process::exit(0x5);
                }
            }
        }

        let mut settings_file = match File::open(&settings_path) {
            Ok(file) => {
                let reader = BufReader::new(file);

                match serde_json::from_reader(reader) {
                    Ok(val) => val,
                    Err(e) => {
                        log!("Error parsing settings file. Using defaults: {}", e);
                        SettingsFile::default()
                    }
                }
            }
            Err(e) => {
                println!("Failed to read {:?}: {}", settings_path, e);
                std::process::exit(0x5);
            }
        };

        log!("Using root directory {:?}", path);

        SettingsService {
            path,
            settings_path,
            atlas_cache_reading: true,
            atlas_cache_writing: true,
            debug_vertices: false,
            debug_atlas: false,
            backface_culling: true,
            chunk_edge_faces: false,
            debug_states: true,
            config: settings_file,
        }
    }

    /// Save the settings service
    pub fn save(&self) {
        match File::create(&self.settings_path) {
            Ok(mut file) => {
                if let Err(e) = file.write(
                    serde_json::to_string_pretty(&self.config)
                        .unwrap()
                        .as_bytes(),
                ) {
                    log_error!("Failed to write {:?}: {}", self.settings_path, e);
                }
            }
            Err(e) => {
                log_error!("Failed to write {:?}: {}", self.settings_path, e);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsFile {
    pub ssao: bool,
    pub bloom: bool,
    pub fullscreen: bool,
    pub render_distance: u32,
}

impl Default for SettingsFile {
    fn default() -> Self {
        SettingsFile {
            ssao: true,
            bloom: true,
            fullscreen: false,
            render_distance: 6,
        }
    }
}
