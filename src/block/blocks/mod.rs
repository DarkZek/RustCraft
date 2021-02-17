use crate::block::{Block, BlockModel, ToolType};
use crate::services::asset_service::atlas::atlas_update_blocks;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::chunk::Color;
use include_dir::{include_dir, Dir};
use lazy_static::LazyStatic;
use nalgebra::Vector3;
use serde_json::Value;
use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fs;
use std::lazy::SyncOnceCell;
use std::sync::Arc;

pub static BLOCK_STATES: SyncOnceCell<BlockStates> = SyncOnceCell::new();

pub struct BlockStates {
    pub states: Vec<BlockStateEntry>,
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct BlockStateEntry {
    block_id: String,
    block_number: usize,
    properties: HashMap<String, Value>,
}

impl BlockStates {
    pub fn get_block(&self, id: usize) -> Option<&Block> {
        self.blocks.get(self.states.get(id).unwrap().block_number)
    }

    pub fn generate(asset: &AssetService) {
        use std::str;

        let mut states: Vec<BlockStateEntry> = vec![];
        let mut blocks = Vec::new();

        let blocks_list = include_str!("../../../assets/blocks.list");

        let blockstate_files: Dir = include_dir!("./assets/minecraft/blockstates/");
        let models_files: Dir = include_dir!("./assets/minecraft/models/");

        let json = serde_json::from_str::<Vec<Value>>(&blocks_list).unwrap();

        for (i, val) in json.iter().enumerate() {
            let id = val["name"].to_string();

            let cull = id.find(":").unwrap() + 1;
            let block = id
                .chars()
                .skip(cull)
                .take(id.len() - cull - 1)
                .collect::<String>();

            let file: include_dir::File = blockstate_files
                .get_file(format!("{}.json", block))
                .unwrap();

            let contents = str::from_utf8(file.contents()).unwrap();

            let block_states = serde_json::from_str::<Value>(&contents);

            // Get first model name until we sort out model loading
            let texture = {
                let (_, tex) = contents.split_at(contents.find("\"model\": \"").unwrap() + 10);
                let (tex, _) = tex.split_at(tex.find("\"").unwrap());
                tex
            };

            // Get texture from model
            let texture_name = {
                let (dir_name, _) = texture.split_at(texture.find("/").unwrap() as usize + 1);

                let dir = models_files.get_dir(dir_name).unwrap();

                let json = str::from_utf8(
                    dir.get_file(format!("{}.json", texture))
                        .unwrap()
                        .contents(),
                )
                .unwrap_or("{}");

                let json = serde_json::from_str::<Value>(json).unwrap();

                match json.get("textures") {
                    None => String::from("mcv3/error"),
                    Some(textures) => {
                        let mut value = "";
                        for (key, val) in textures.as_object().unwrap() {
                            value = val.as_str().unwrap();
                            break;
                        }
                        String::from(value)
                    }
                }
            };

            let resource = Block {
                name: Some(texture_name.clone()),
                raw_texture_names: [
                    texture_name.clone(),
                    texture_name.clone(),
                    texture_name.clone(),
                    texture_name.clone(),
                    texture_name.clone(),
                    texture_name.clone(),
                ],
                texture_atlas_lookups: [([0.0; 2], [0.0; 2]); 6],
                color: [0; 3],
                light_intensity: 0,
                transparent: if i == 34 || i == 230 || i == 1341 || (i >= 1342 && i <= 1346) {
                    print!("test{}", i);
                    true
                } else {
                    false
                },
            };

            blocks.push(resource);

            let states_len = calculate_states_len(&val);

            for _ in 0..states_len {
                states.push(BlockStateEntry {
                    block_id: String::from(block.clone()),
                    block_number: i,
                    properties: Default::default(),
                })
            }
        }

        atlas_update_blocks(asset.atlas_index.as_ref().unwrap(), &mut blocks);

        unsafe {
            BLOCK_STATES.set(BlockStates { states, blocks });
        }
    }

    pub fn get_id() {}
}

fn calculate_states_len(value: &Value) -> usize {
    if value["properties"] == Value::Null {
        return 1;
    }

    let mut ids = 0;
    for (_, num) in value.get("properties").unwrap().as_object().unwrap() {
        let size = num.as_array().unwrap().len();

        if ids == 0 {
            ids = size;
        } else {
            ids *= size;
        }
    }

    return ids;
}

#[derive(Clone)]
pub struct BlockResource {
    pub id: u16,
    pub identifier: String,
    pub palette_id: u32,
    pub full: bool, // Collision purposes
    pub gravity: bool,
    pub collidable: bool,
    pub hardness: f32,
    pub stack_size: u32,
    pub blast_resistance: f32,
    pub transparent: bool,
    pub tool: ToolType,
    pub model: BlockModel,
    pub inventory_size: u32,
    pub particles: Option<(i32, i32, Vector3<f32>)>,

    // Index ID to the traits array
    pub traits: Option<i32>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait BlockTraits {
    fn get_light_level(&self) -> Color { return [0; 4]; }

    fn on_right_click(&self) {}
    fn on_left_click(&self) {} // Note blocks
    fn on_block_update(&self) {}
    fn on_try_place(&self) {} // For stuff that cant be placed just anywhere like cactus, sugarcane
}
