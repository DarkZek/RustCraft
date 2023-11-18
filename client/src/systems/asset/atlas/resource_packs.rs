use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use bevy::asset::Asset;
use bevy::reflect::TypePath;
use std::path::PathBuf;

/// A listing of all resource packs available to the program
#[derive(Asset, Debug, Clone, TypeUuid, Deserialize, Serialize, TypePath)]
#[uuid = "7b14806a-672b-443b-8d16-4f18afefa463"]
pub struct ResourcePacks {
    list: Vec<ResourcePack>,
}

impl ResourcePacks {
    pub fn new(list: Vec<ResourcePack>) -> ResourcePacks {
        ResourcePacks { list }
    }

    pub fn get_default(&self) -> &ResourcePack {
        self.list.get(0).unwrap()
    }
}

impl Default for ResourcePacks {
    fn default() -> Self {
        ResourcePacks {
            list: vec![ResourcePack {
                name: String::from("Default Pack"),
                path: PathBuf::from("resources.pack"),
            }],
        }
    }
}

// An item listing a resource packs name and path to its resources
#[derive(Debug, Clone, TypeUuid, Deserialize, Serialize)]
#[uuid = "7b14806a-632b-443b-8d16-4f18afefa463"]
pub struct ResourcePack {
    pub name: String,
    pub path: PathBuf,
}
