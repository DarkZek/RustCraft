use std::path::PathBuf;

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
    pub path: PathBuf,
    pub atlas_cache_reading: bool,
    pub atlas_cache_writing: bool,
    pub render_distance: u32,
    /// Changes the texture atlas to generate random textures instead
    pub debug_vertices: bool,
    pub debug_atlas: bool,
    pub backface_culling: bool,
    pub chunk_edge_faces: bool,
    pub debug_states: bool,
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
        let mut path = dirs::data_local_dir().unwrap();
        path.push("RustCraft");

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
            backface_culling: true,
            chunk_edge_faces: true,
            debug_states: true,
        }
    }
}
