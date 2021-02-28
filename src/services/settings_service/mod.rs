use std::fs;

#[cfg(not(target_os = "windows"))]
#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
}

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
    pub debug_states: bool,
}

impl Default for SettingsService {
    fn default() -> Self {
        unimplemented!()
    }
}

impl SettingsService {
    #[cfg(target_os = "windows")]
    fn get_path() -> String {
        let path = std::env::current_dir().unwrap();
        let path = path.as_os_str();
        String::from(path.to_str().unwrap()).add("\\")
    }

    #[cfg(not(target_os = "windows"))]
    fn get_path() -> String {
        let path = std::env::var("HOME");

        let path = if let Result::Ok(val) = path {
            val
        } else {
            // Backup method, checking /etc/passwd
            let file = fs::read_to_string("/etc/passwd").expect("Failed to find home directory, set environment variable HOME or specify in /etc/passwd");
            let mut path = String::new();
            for line in file.split("\n") {
                let params = line.split(":").collect::<Vec<&str>>();
                unsafe {
                    if params
                        .get(2)
                        .expect("Corrupt /etc/passwd")
                        .parse::<u32>()
                        .expect("Corrupt /etc/passwd")
                        == geteuid()
                    {
                        path = params.get(5).unwrap().to_string();
                        break;
                    }
                }
            }
            if &path == "" {
                log_error!("Failed to find home directory");
                panic!("Failed to find home directory");
            }

            path
        };

        format!("{}/.rustcraft/", path)
    }

    /// Creates a new instance of settings, loading the variables from the environment variables as well as settings file
    ///
    /// # Returns
    ///
    /// A new initialized `SettingsService`
    ///
    pub fn new() -> SettingsService {
        // Load resources directory
        let path: String = Self::get_path();

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
            debug_states: false,
        }
    }
}
