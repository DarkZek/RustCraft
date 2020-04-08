use std::ops::Add;

pub struct SettingsService {
    pub(crate) path: String,
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

        SettingsService {
            path
        }
    }
}