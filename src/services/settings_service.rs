//
// Handles the state of settings, loading settings from json and automatically updating settings when necessary
//

use std::ops::Add;

pub const CHUNK_SIZE: usize = 16;

pub struct SettingsService {
    pub path: String,
    //TODO: Implement
    pub atlas_cache_reading: bool,
    pub atlas_cache_writing: bool,
    pub render_distance: u32,
    /// Changes the texture atlas to generate random textures instead
    pub debug_vertices: bool,
    pub debug_atlas: bool,
    pub backface_culling: bool,
    pub chunk_edge_faces: bool
}

impl SettingsService {
    pub fn new() -> SettingsService {
        // Load resources directory
        let path: String = {
            let path = std::env::current_exe().unwrap();
            let path = path.as_os_str();
            let mut path: Vec<&str> = (path.to_str().unwrap()).split("/").collect();

            path.remove(path.len() - 1);
            path.iter().fold("".to_string(), | out, x| { out.add(&format!("{}/", x)) })
        };

        let mut atlas_caching = true;
        let debug_vertices = false;

        if debug_vertices {
            atlas_caching = false;
        }

        SettingsService {
            path,
            atlas_cache_reading: true,
            atlas_cache_writing: true,
            render_distance: 24,
            debug_vertices,
            debug_atlas: false,
            backface_culling: true,
            chunk_edge_faces: false
        }
    }
}