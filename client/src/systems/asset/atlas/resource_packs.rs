use crate::systems::asset::atlas::AtlasLoadingStage;
use crate::systems::asset::AssetService;
use bevy::asset::{Asset, Assets};
use bevy::log::error;
use bevy::prelude::{AssetServer, DetectChanges, Res, ResMut};
use bevy::reflect::TypePath;
use bevy::reflect::TypeUuid;
use fnv::FnvBuildHasher;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
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

// An type listing a resource packs name and path to its resources
#[derive(Debug, Clone, TypeUuid, Deserialize, Serialize)]
#[uuid = "7b14806a-632b-443b-8d16-4f18afefa463"]
pub struct ResourcePack {
    pub name: String,
    pub path: PathBuf,
}

/// The images that make up a resource pack
#[derive(Asset, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "7b14806a-672b-423b-8d16-4f18afefa463"]
pub struct ResourcePackData {
    pub images: HashMap<String, DynamicImage, FnvBuildHasher>,
}

impl ResourcePackData {
    pub fn new(images: HashMap<String, DynamicImage, FnvBuildHasher>) -> ResourcePackData {
        ResourcePackData { images }
    }
}

pub fn load_resource_zips(
    packs: Res<Assets<ResourcePacks>>,
    mut service: ResMut<AssetService>,
    server: Res<AssetServer>,
    mut stage: ResMut<AtlasLoadingStage>,
) {
    // Only load zips on change to resource packs
    if *stage != AtlasLoadingStage::AwaitingIndex || packs.len() == 0 {
        return;
    }
    if !packs.is_changed() {
        return;
    }

    let packs = packs.get(&service.resource_packs).unwrap();

    let pack = packs.get_default();

    if !pack.path.extension().unwrap().eq(&OsString::from("pack")) {
        error!(
            "Resource pack {:?} does not end with .pack, aborting load.",
            pack.path
        );
        return;
    }

    service.pack = Some(server.load(pack.path.clone()));

    *stage = AtlasLoadingStage::AwaitingPack;
}
