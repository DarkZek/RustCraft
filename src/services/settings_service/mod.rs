use std::ops::Add;

pub mod key_mappings;

pub const CHUNK_SIZE: usize = 16;

//TODO: Make it actually load from a text file

/// Responsible for loading settings from environment variables, loading settings file and saving the settings file. Giving us persistent storage for variables.
/// This is separate from the loading of world files, it only operates in text.
///
/// ## Loading
///
/// Loads settings file from only the local directory the executable is ran from.
///
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
    pub chunk_edge_faces: bool,
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
        let path: String = {
            let path = std::env::current_exe().unwrap();
            let path = path.as_os_str();
            let mut path: Vec<&str> = (path.to_str().unwrap()).split("/").collect();

            path.remove(path.len() - 1);
            path.iter()
                .fold("".to_string(), |out, x| out.add(&format!("{}/", x)))
        };

        let mut atlas_caching = true;
        let debug_vertices = false;

        if debug_vertices {
            atlas_caching = false;
        }

        SettingsService {
            path,
            atlas_cache_reading: true,
            atlas_cache_writing: atlas_caching,
            render_distance: 6,
            debug_vertices,
            debug_atlas: false,
            backface_culling: false,
            chunk_edge_faces: false,
        }
    }
}
