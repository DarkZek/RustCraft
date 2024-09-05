use bevy::prelude::{Resource};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Resource)]
pub struct ServerConfig {
    pub port: u16,
    pub save_world: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: 25568,
            save_world: true,
        }
    }
}

pub fn load_config() -> ServerConfig {
    if !fs::exists("settings.json").unwrap() {
        let file = File::create("settings.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &ServerConfig::default()).unwrap();
    }

    let settings = if let Ok(file) = File::open("settings.json") {
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).ok()
    } else {
        None
    };

    settings.unwrap_or(ServerConfig::default())
}
