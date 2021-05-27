use crate::block::blocks::model::{BlockFace, BlockModel};
use crate::block::Block;
use crate::game::physics::collider::BoxCollider;
use crate::helpers::{AtlasIndex, TextureSubdivisionMethod};
use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::mesh::ViewableDirectionBitMap;
use crate::services::settings_service::SettingsService;
use nalgebra::{Point, Vector3};
use serde_json::Value;
use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::ops::Add;

pub mod model;

pub static BLOCK_STATES: SyncOnceCell<BlockStates> = SyncOnceCell::new();

pub struct BlockStates {
    pub states: Vec<BlockStateEntry>,
    pub blocks: Vec<BlockType>,
}

#[derive(Debug)]
pub struct BlockStateEntry {
    block_number: usize,
    properties: Option<HashMap<String, PropertyType>>,
}

impl BlockStates {
    pub fn get_block(&self, id: usize) -> Option<Block> {
        match self.states.get(id) {
            None => None,
            Some(block_state) => Some(Block {
                block_type: &self.blocks.get(block_state.block_number).unwrap(),
                block_state_index: id,
            }),
        }
    }

    pub fn generate(asset: &AssetService, settings: &SettingsService) {
        let states = get_blockstates();
        let blocks = get_blocks();

        if settings.debug_states {
            let mut str = String::new();
            for (i, state) in states.iter().enumerate() {
                str = str.add(&*format!(
                    "{}: {}\n",
                    i,
                    blocks.get(state.block_number).unwrap().get_identifier()
                ));
            }
            if let Result::Err(error) =
                std::fs::write(format!("{}cache/debug_states.txt", settings.path), str)
            {
                log_error!("Error writing debug states: {}", error);
            }
        }

        if let Result::Err(_) = BLOCK_STATES.set(BlockStates { states, blocks }) {
            log_error!("Error setting block states list")
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum PropertyType {
    Boolean(bool),
    TreeVariant(TreeVariant),
    UnsignedByte(u8),
    Axis(Axis),
    Direction(Direction),
    Instrument(Instrument),
    RailShape(RailShape),
    StairShape(StairShape),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TreeVariant {
    Oak,
    Spruce,
    Birch,
    Jungle,
    Acacia,
    DarkOak,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Instrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum StairShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl Default for TreeVariant {
    fn default() -> Self {
        TreeVariant::Oak
    }
}
impl Default for Axis {
    fn default() -> Self {
        Axis::X
    }
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Up
    }
}
impl Default for Instrument {
    fn default() -> Self {
        Instrument::Banjo
    }
}
impl Default for RailShape {
    fn default() -> Self {
        RailShape::AscendingEast
    }
}
impl Default for StairShape {
    fn default() -> Self {
        StairShape::Straight
    }
}

fn append_states(
    states: &mut Vec<BlockStateEntry>,
    block: &Vec<(&str, Vec<PropertyType>)>,
    block_number: usize,
    current_properties: HashMap<String, PropertyType>,
    mut depth: usize,
) {
    if block.len() == 0 {
        states.push(BlockStateEntry {
            block_number: block_number.clone(),
            properties: None,
        });
        return;
    }
    let (name, properties) = block.get(depth).unwrap();
    depth += 1;

    for property in properties {
        let mut properties = current_properties.clone();
        properties.insert(name.to_string(), property.clone());

        if depth == block.len() {
            states.push(BlockStateEntry {
                block_number: block_number.clone(),
                properties: Some(properties),
            });
        } else {
            append_states(
                states,
                block,
                block_number,
                current_properties.clone(),
                depth,
            );
        }
    }
}

#[macro_export]
macro_rules! define_blocks {
    (
        $(
            $name:ident {
                $(identifier $identifierfunc:expr,)?
                props {
                    $(
                        $fname:ident: $ftype:ty = $fvalue:expr,
                    )*
                },
                $(model $model:expr,)?
                $(collidable $collidable:expr,)?
                $(full $full:expr,)?
                $(stacksize $stacksize:expr,)?
                $(light_color $light_color:expr,)?
                $(light_intensity $light_intensity:expr,)?
                $(transparent $transparent:expr,)?
                $(waterlogged $waterlogged:expr,)?
            }
        )+
    ) => (
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum BlockType {
            $(
                $name {
                    $(
                        $fname: $ftype,
                    )?
                },
            )+
        }

        #[allow(unused_variables, unreachable_code)]
        pub fn get_blocks() -> Vec<BlockType> {
            vec![
                $(
                    BlockType::$name {
                        $(
                            $fname: Default::default(),
                        )?
                    },
                )+
            ]
        }

        #[allow(unused_variables, unreachable_code)]
        pub fn get_blockstates() -> Vec<BlockStateEntry> {
            let blocks = vec![
                $(
                    vec![
                        $(
                            (stringify!($fname), ($fvalue)),
                        )?
                    ],
                )+
            ];

            let mut states = Vec::new();

            for (i, block) in blocks.iter().enumerate() {
                append_states(
                    &mut states,
                    &block,
                    i,
                    HashMap::new(),
                    0,
                );
            }

            states
        }

        impl BlockType {
            #[allow(unused_variables, unreachable_code)]
            pub fn get_collision_boxes(&self) -> Vec<BoxCollider> {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(
                                if !$collidable {
                                    return vec![];
                                }

                            )?
                            return vec![
                                BoxCollider {
                                    p1: Point { coords: Vector3::new(0.0, 0.0, 0.0) },
                                    p2: Point { coords: Vector3::new(0.0, 0.0, 0.0) },
                                    center: Point { coords: Vector3::new(0.0, 0.0, 0.0) },
                                }
                            ];
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn is_block_full(&self) -> bool {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $full;)?
                            return true;
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn is_waterlogged(&self) -> bool {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $waterlogged;)?
                            return false;
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_identifier(&self) -> &str {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $identifierfunc;)?
                            return "mcv3:none";
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_stacksize(&self) -> u16 {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $stacksize;)?
                            return 64;
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_light_color(&self) -> [u8; 3] {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $light_color;)?
                            return [0; 3];
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_light_intensity(&self) -> u8 {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $light_intensity;)?
                            return 0;
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_transparency(&self) -> bool {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $transparent;)?
                            return false;
                        }
                    )+
                }
            }

            #[allow(unused_variables, unreachable_code)]
            pub fn get_model(&self, atlas_lookups: &HashMap<String, TextureAtlasIndex>) -> BlockModel {
                match *self {
                    $(
                        BlockType::$name {
                            $($fname,)?
                        } => {
                            $(return $model;)?
                            return BlockModel::square_block(["mcv3/error"; 6]);
                        }
                    )+
                }
            }
        }
    );
}

define_blocks! {
    Air {
        identifier "minecraft:air",
        props {},
        collidable false,
        full false,
    }
    Stone {
        identifier "minecraft:stone",
        props {},
        model BlockModel::square_block(["block/stone"; 6]),
    }
    Granite {
        identifier "minecraft:granite",
        props {},
        model BlockModel::square_block(["block/granite"; 6]),
    }
    SmoothGranite {
        identifier "minecraft:smooth_granite",
        props {},
        model BlockModel::square_block(["block/polished_granite"; 6]),
    }
    Diorite {
        identifier "minecraft:diorite",
        props {},
        model BlockModel::square_block(["block/diorite"; 6]),
    }
    SmoothDiorite {
        identifier "minecraft:smooth_diorite",
        props {},
        model BlockModel::square_block(["block/polished_diorite"; 6]),
    }
    Andesite {
        identifier "minecraft:andesite",
        props {},
        model BlockModel::square_block(["block/andesite"; 6]),
    }
    SmoothAndesite {
        identifier "minecraft:smooth_andesite",
        props {},
        model BlockModel::square_block(["block/polished_andesite"; 6]),
    }
    GrassBlock {
        identifier "minecraft:grass_block",
        props {
            snowy: bool = vec![
                    PropertyType::Boolean(true),
                    PropertyType::Boolean(false)
                ],
        },
        model {
            BlockModel::square_block(["block/grass_block_top", "block/dirt", "block/grass_block_side", "block/grass_block_side", "block/grass_block_side", "block/grass_block_side"])
        },
    }
    Dirt {
        identifier "minecraft:dirt",
        props {},
        model BlockModel::square_block(["block/dirt"; 6]),
    }
    CoarseDirt {
        identifier "minecraft:coarse_dirt",
        props {},
        model BlockModel::square_block(["block/coarse_dirt"; 6]),
    }
    Podzol {
        identifier "minecraft:podzol",
        props {
            snowy: bool = vec![
                    PropertyType::Boolean(true),
                    PropertyType::Boolean(false)
                ],
        },
        model BlockModel::square_block(["block/podzol_top", "block/dirt", "block/podzol_side", "block/podzol_side", "block/podzol_side", "block/podzol_side"]),
    }
    Cobblestone {
        identifier "minecraft:cobblestone",
        props {},
        model BlockModel::square_block(["block/cobblestone"; 6]),
    }
    Planks {
        identifier "minecraft:wooden_planks",
        props {
            variant: TreeVariant = vec![
                PropertyType::TreeVariant(TreeVariant::Oak),
                PropertyType::TreeVariant(TreeVariant::Spruce),
                PropertyType::TreeVariant(TreeVariant::Birch),
                PropertyType::TreeVariant(TreeVariant::Jungle),
                PropertyType::TreeVariant(TreeVariant::Acacia),
                PropertyType::TreeVariant(TreeVariant::DarkOak)
            ],
        },
        model BlockModel::square_block(["block/oak_planks"; 6]),
    }
    OakSapling {
        identifier "minecraft:oak_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/oak_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    SpruceSapling {
        identifier "minecraft:spruce_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/spruce_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    BirchSapling {
        identifier "minecraft:birch_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/birch_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    JungleSapling {
        identifier "minecraft:jungle_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/jungle_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    AcaciaSapling {
        identifier "minecraft:acacia_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/acacia_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    DarkOakSapling {
        identifier "minecraft:dark_oak_sapling",
        props {
            stage: u8 = vec![PropertyType::UnsignedByte(0), PropertyType::UnsignedByte(1)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/dark_oak_sapling").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
    }
    Bedrock {
        identifier "minecraft:bedrock",
        props {},
        model BlockModel::square_block(["block/bedrock"; 6]),
    }
    Water {
        identifier "minecraft:water",
        props {
            water: u8 = vec![PropertyType::UnsignedByte(0),
            PropertyType::UnsignedByte(1),
            PropertyType::UnsignedByte(2),
            PropertyType::UnsignedByte(3),
            PropertyType::UnsignedByte(4),
            PropertyType::UnsignedByte(5),
            PropertyType::UnsignedByte(6),
            PropertyType::UnsignedByte(7),
            PropertyType::UnsignedByte(8),
            PropertyType::UnsignedByte(9),
            PropertyType::UnsignedByte(10),
            PropertyType::UnsignedByte(11),
            PropertyType::UnsignedByte(12),
            PropertyType::UnsignedByte(13),
            PropertyType::UnsignedByte(14),
            PropertyType::UnsignedByte(15)],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/water_still").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.85, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Top,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Bottom,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Left,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Right,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Front,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: lookup.clone(),
                        normal: ViewableDirectionBitMap::Back,
                        edge: true,
                    },
                ]
            }
        },
        collidable false,
        transparent true,
        waterlogged true,
    }
    Lava {
        identifier "minecraft:lava",
        props {
            water: u8 = vec![PropertyType::UnsignedByte(0),
            PropertyType::UnsignedByte(1),
            PropertyType::UnsignedByte(2),
            PropertyType::UnsignedByte(3),
            PropertyType::UnsignedByte(4),
            PropertyType::UnsignedByte(5),
            PropertyType::UnsignedByte(6),
            PropertyType::UnsignedByte(7),
            PropertyType::UnsignedByte(8),
            PropertyType::UnsignedByte(9),
            PropertyType::UnsignedByte(10),
            PropertyType::UnsignedByte(11),
            PropertyType::UnsignedByte(12),
            PropertyType::UnsignedByte(13),
            PropertyType::UnsignedByte(14),
            PropertyType::UnsignedByte(15)],
        },
        model BlockModel::square_block(["block/lava_still"; 6]),
        collidable false,
        transparent true,
    }
    Sand {
        identifier "minecraft:sand",
        props {},
        model BlockModel::square_block(["block/sand"; 6]),
    }
    RedSand {
        identifier "minecraft:red_sand",
        props {},
        model BlockModel::square_block(["block/red_sand"; 6]),
    }
    Gravel {
        identifier "minecraft:gravel",
        props {},
        model BlockModel::square_block(["block/gravel"; 6]),
    }
    GoldOre {
        identifier "minecraft:gold_ore",
        props {},
        model BlockModel::square_block(["block/gold_ore"; 6]),
    }
    IronOre {
        identifier "minecraft:iron_ore",
        props {},
        model BlockModel::square_block(["block/iron_ore"; 6]),
    }
    CoalOre {
        identifier "minecraft:gravel",
        props {},
        model BlockModel::square_block(["block/coal_ore"; 6]),
    }
    OakLog {
        identifier "minecraft:oak_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/oak_log_top", "block/oak_log_top", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log"]),
    }
    SpruceLog {
        identifier "minecraft:spruce_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/spruce_log_top", "block/spruce_log_top", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log"]),
    }
    BirchLog {
        identifier "minecraft:birch_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/birch_log_top", "block/birch_log_top", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log"]),
    }
    JungleLog {
        identifier "minecraft:jungle_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/jungle_log_top", "block/jungle_log_top", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log"]),
    }
    AcaciaLog {
        identifier "minecraft:acacia_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/acacia_log_top", "block/acacia_log_top", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log"]),
    }
    DarkOakLog {
        identifier "minecraft:dark_oak_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/dark_oak_log_top", "block/dark_oak_log_top", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log"]),
    }
    StrippedSpruceLog {
        identifier "minecraft:stripped_spruce_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_spruce_log_top", "block/stripped_spruce_log_top", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log"]),
    }
    StrippedBirchLog {
        identifier "minecraft:stripped_birch_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_birch_log_top", "block/stripped_birch_log_top", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log"]),
    }
    StrippedJungleLog {
        identifier "minecraft:stripped_jungle_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_jungle_log_top", "block/stripped_jungle_log_top", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log"]),
    }
    StrippedAcaciaLog {
        identifier "minecraft:stripped_acacia_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_acacia_log_top", "block/stripped_acacia_log_top", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log"]),
    }
    StrippedDarkOakLog {
        identifier "minecraft:stripped_dark_oak_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_dark_oak_log_top", "block/stripped_dark_oak_log_top", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log"]),
    }
    StrippedOakLog {
        identifier "minecraft:stripped_oak_log",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_oak_log_top", "block/stripped_oak_log_top", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log"]),
    }
    OakWood {
        identifier "minecraft:oak_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log"]),
    }
    SpruceWood {
        identifier "minecraft:spruce_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log"]),
    }
    BirchWood {
        identifier "minecraft:birch_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log"]),
    }
    JungleWood {
        identifier "minecraft:jungle_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log"]),
    }
    AcaciaWood {
        identifier "minecraft:acacia_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log"]),
    }
    DarkOakWood {
        identifier "minecraft:jungle_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log"]),
    }
    StrippedOakWood {
        identifier "minecraft:stripped_oak_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log"]),
    }
    StrippedSpruceWood {
        identifier "minecraft:stripped_spruce_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log"]),
    }
    StrippedBirchWood {
        identifier "minecraft:stripped_birch_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log"]),
    }
    StrippedJungleWood {
        identifier "minecraft:stripped_jungle_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log"]),
    }
    StrippedAcaciaWood {
        identifier "minecraft:stripped_acacia_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log"]),
    }
    StrippedDarkOakWood {
        identifier "minecraft:stripped_jungle_wood",
        props {
            axis: Axis = vec![PropertyType::Axis(Axis::X), PropertyType::Axis(Axis::Y), PropertyType::Axis(Axis::Z)],
        },
        model BlockModel::square_block(["block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log"]),
    }
    OakLeaves {
        identifier "minecraft:oak_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves"]),
        transparent true,
    }
    SpruceLeaves {
        identifier "minecraft:spruce_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves"]),
        transparent true,
    }
    BirchLeaves {
        identifier "minecraft:birch_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves"]),
        transparent true,
    }
    JungleLeaves {
        identifier "minecraft:jungle_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves"]),
        transparent true,
    }
    AcaciaLeaves {
        identifier "minecraft:acacia_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves"]),
        transparent true,
    }
    DarkOakLeaves {
        identifier "minecraft:dark_oak_leaves",
        props {
            distance: u8 = vec![
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7)
                ],
            persistent: bool = vec![PropertyType::Boolean(true), PropertyType::Boolean(false)],
        },
        model BlockModel::square_block(["block/dark_oak_leaves"; 6]),
        transparent true,
    }
    Sponge {
        identifier "minecraft:sponge",
        props {},
        model BlockModel::square_block(["block/sponge"; 6]),
    }
    WetSponge {
        identifier "minecraft:wet_sponge",
        props {},
        model BlockModel::square_block(["block/wet_sponge"; 6]),
    }
    Glass {
        identifier "minecraft:glass",
        props {},
        model BlockModel::square_block(["block/glass"; 6]),
        transparent true,
    }
    LapisOre {
        identifier "minecraft:lapis_ore",
        props {},
        model BlockModel::square_block(["block/lapis_ore"; 6]),
    }
    LapisBlock {
        identifier "minecraft:lapis_block",
        props {},
        model BlockModel::square_block(["block/lapis_block"; 6]),
    }
    Dispenser {
        identifier "minecraft:dispenser",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::East),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::Up),
                PropertyType::Direction(Direction::Down),
            ],
            triggered: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["block/furnace_top","block/furnace_top","block/furnace_top","block/furnace_top","block/dispenser_front","block/furnace_top"]),
    }
    Sandstone {
        identifier "minecraft:sandstone",
        props {},
        model BlockModel::square_block(["block/sandstone_top", "block/sandstone_top", "block/sandstone", "block/sandstone", "block/sandstone", "block/sandstone"]),
    }
    ChiseledSandstone {
        identifier "minecraft:chiseled_sandstone",
        props {},
        model BlockModel::square_block(["block/chiseled_sandstone"; 6]),
    }
    CutSandstone {
        identifier "minecraft:cut_sandstone",
        props {},
        model BlockModel::square_block(["block/cut_sandstone"; 6]),
    }
    NoteBlock {
        identifier "minecraft:note_block",
        props {
            instrument: Instrument = vec![
                PropertyType::Instrument(Instrument::Harp),
                PropertyType::Instrument(Instrument::Basedrum),
                PropertyType::Instrument(Instrument::Snare),
                PropertyType::Instrument(Instrument::Hat),
                PropertyType::Instrument(Instrument::Bass),
                PropertyType::Instrument(Instrument::Flute),
                PropertyType::Instrument(Instrument::Bell),
                PropertyType::Instrument(Instrument::Guitar),
                PropertyType::Instrument(Instrument::Chime),
                PropertyType::Instrument(Instrument::Xylophone),
                PropertyType::Instrument(Instrument::IronXylophone),
                PropertyType::Instrument(Instrument::CowBell),
                PropertyType::Instrument(Instrument::Didgeridoo),
                PropertyType::Instrument(Instrument::Bit),
                PropertyType::Instrument(Instrument::Banjo),
                PropertyType::Instrument(Instrument::Pling)
            ],
            note: u8 = vec![
                PropertyType::UnsignedByte(0),
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7),
                PropertyType::UnsignedByte(8),
                PropertyType::UnsignedByte(9),
                PropertyType::UnsignedByte(10),
                PropertyType::UnsignedByte(11),
                PropertyType::UnsignedByte(12),
                PropertyType::UnsignedByte(13),
                PropertyType::UnsignedByte(14),
                PropertyType::UnsignedByte(15),
                PropertyType::UnsignedByte(16),
                PropertyType::UnsignedByte(17),
                PropertyType::UnsignedByte(18),
                PropertyType::UnsignedByte(19),
                PropertyType::UnsignedByte(20),
                PropertyType::UnsignedByte(21),
                PropertyType::UnsignedByte(22),
                PropertyType::UnsignedByte(23),
                PropertyType::UnsignedByte(24),
            ],
            powered: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["block/note_block"; 6]),
    }
    WhiteBed {
        identifier "minecraft:white_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    OrangeBed {
        identifier "minecraft:orange_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    MagentaBed {
        identifier "minecraft:magenta_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LightBlueBed {
        identifier "minecraft:light_blue_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    YellowBed {
        identifier "minecraft:yellow_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LimeBed {
        identifier "minecraft:lime_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PinkBed {
        identifier "minecraft:pink_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    GrayBed {
        identifier "minecraft:gray_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LightGrayBed {
        identifier "minecraft:light_gray_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    CyanBed {
        identifier "minecraft:cyan_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PurpleBed {
        identifier "minecraft:purple_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BlueBed {
        identifier "minecraft:blue_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BrownBed {
        identifier "minecraft:brown_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    GreenBed {
        identifier "minecraft:green_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    RedBed {
        identifier "minecraft:red_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BlackBed {
        identifier "minecraft:black_bed",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
            occupied: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            head_part: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PoweredRail {
        identifier "minecraft:powered_rail",
        props {
            powered: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            shape: Direction = vec![
                PropertyType::RailShape(RailShape::NorthSouth),
                PropertyType::RailShape(RailShape::EastWest),
                PropertyType::RailShape(RailShape::AscendingEast),
                PropertyType::RailShape(RailShape::AscendingWest),
                PropertyType::RailShape(RailShape::AscendingNorth),
                PropertyType::RailShape(RailShape::AscendingSouth),
            ],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/powered_rail").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Top,
                            edge: false,
                        }
                    ],
            }
        },
        collidable false,
        full false,
    }
    DetectorRail {
        identifier "minecraft:detector_rail",
        props {
            powered: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            shape: Direction = vec![
                PropertyType::RailShape(RailShape::NorthSouth),
                PropertyType::RailShape(RailShape::EastWest),
                PropertyType::RailShape(RailShape::AscendingEast),
                PropertyType::RailShape(RailShape::AscendingWest),
                PropertyType::RailShape(RailShape::AscendingNorth),
                PropertyType::RailShape(RailShape::AscendingSouth),
            ],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/detector_rail").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Top,
                            edge: false,
                        }
                    ]
            }
        },
        collidable false,
        full false,
    }
    StickyPiston {
        identifier "minecraft:sticky_piston",
        props {
            extended: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::East),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::Up),
                PropertyType::Direction(Direction::Down),
            ],
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top_sticky", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    Cobweb {
        identifier "minecraft:cobweb",
        props {},
        model BlockModel::square_block(["block/cobweb"; 6]),
        transparent true,
    }
    Grass {
        identifier "minecraft:grass",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/grass").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        transparent true,
    }
    Fern {
        identifier "minecraft:fern",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/fern").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        transparent true,
    }
    DeadBush {
        identifier "minecraft:dead_bush",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/dead_bush").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    Seagrass {
        identifier "minecraft:seagrass",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/seagrass").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            let water_lookup = ATLAS_LOOKUPS.get().unwrap().get("block/water_still").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        //TODO: Scale this correctly, the texture is being smushed down
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.85, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.85, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 0.85, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 0.85, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        // Water
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.85, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Top,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Bottom,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Front,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 1.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Back,
                            edge: true,
                        },
                ]
            }
        },
        transparent true,
        waterlogged true,
    }
    TallSeagrass {
        identifier "minecraft:tall_seagrass",
        props {
            upper_half: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/tall_seagrass_bottom").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            let water_lookup = ATLAS_LOOKUPS.get().unwrap().get("block/water_still").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        // Water
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.85, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Top,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Bottom,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Front,
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 1.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            normal: ViewableDirectionBitMap::Back,
                            edge: true,
                        },
                ],
            }
        },
        transparent true,
        waterlogged true,
    }
    Piston {
        identifier "minecraft:piston",
        props {
            extended: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::East),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::Up),
                PropertyType::Direction(Direction::Down),
            ],
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    PistonHead {
        identifier "minecraft:piston_head",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::East),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::Up),
                PropertyType::Direction(Direction::Down),
            ],
            short: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            is_sticky: bool = vec![
                PropertyType::Boolean(false),
                PropertyType::Boolean(true),
            ],
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    WhiteWool {
        identifier "minecraft:white_wool",
        props {},
        model BlockModel::square_block(["block/white_wool"; 6]),
    }
    OrangeWool {
        identifier "minecraft:orange_wool",
        props {},
        model BlockModel::square_block(["block/orange_wool"; 6]),
    }
    MagentaWool {
        identifier "minecraft:magenta_wool",
        props {},
        model BlockModel::square_block(["block/magenta_wool"; 6]),
    }
    LightBlueWool {
        identifier "minecraft:light_blue_wool",
        props {},
        model BlockModel::square_block(["block/light_blue_wool"; 6]),
    }
    YellowWool {
        identifier "minecraft:yellow_wool",
        props {},
        model BlockModel::square_block(["block/yellow_wool"; 6]),
    }
    LimeWool {
        identifier "minecraft:lime_wool",
        props {},
        model BlockModel::square_block(["block/lime_wool"; 6]),
    }
    PinkWool {
        identifier "minecraft:pink_wool",
        props {},
        model BlockModel::square_block(["block/pink_wool"; 6]),
    }
    GrayWool {
        identifier "minecraft:gray_wool",
        props {},
        model BlockModel::square_block(["block/gray_wool"; 6]),
    }
    LightGrayWool {
        identifier "minecraft:light_gray_wool",
        props {},
        model BlockModel::square_block(["block/light_gray_wool"; 6]),
    }
    CyanWool {
        identifier "minecraft:cyan_wool",
        props {},
        model BlockModel::square_block(["block/cyan_wool"; 6]),
    }
    PurpleWool {
        identifier "minecraft:purple_wool",
        props {},
        model BlockModel::square_block(["block/purple_wool"; 6]),
    }
    BlueWool {
        identifier "minecraft:blue_wool",
        props {},
        model BlockModel::square_block(["block/blue_wool"; 6]),
    }
    BrownWool {
        identifier "minecraft:brown_wool",
        props {},
        model BlockModel::square_block(["block/brown_wool"; 6]),
    }
    GreenWool {
        identifier "minecraft:green_wool",
        props {},
        model BlockModel::square_block(["block/green_wool"; 6]),
    }
    RedWool {
        identifier "minecraft:red_wool",
        props {},
        model BlockModel::square_block(["block/red_wool"; 6]),
    }
    BlackWool {
        identifier "minecraft:black_wool",
        props {},
        model BlockModel::square_block(["block/black_wool"; 6]),
    }
    MovingPiston {
        identifier "minecraft:moving_piston",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::East),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::Up),
                PropertyType::Direction(Direction::Down),
            ],
            is_sticky: bool = vec![
                PropertyType::Boolean(false),
                PropertyType::Boolean(true),
            ],
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
    }
    Dandelion {
        identifier "minecraft:dandelion",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/dandelion").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    Poppy {
        identifier "minecraft:poppy",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/poppy").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ],
            }
        },
        transparent true,
    }
    BlueOrchid {
        identifier "minecraft:blue_orchid",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/blue_orchid").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    Allium {
        identifier "minecraft:allium",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/allium").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    AzureBluet {
        identifier "minecraft:azure_bluet",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/azure_bluet").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    RedTulip {
        identifier "minecraft:red_tulip",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/red_tulip").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    OrangeTulip {
        identifier "minecraft:orange_tulip",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/orange_tulip").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    WhiteTulip {
        identifier "minecraft:white_tulip",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/white_tulip").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    PinkTulip {
        identifier "minecraft:pink_tulip",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/pink_tulip").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    OxeyeDaisy {
        identifier "minecraft:oxeye_daisy",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/oxeye_daisy").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    Cornflower {
        identifier "minecraft:cornflower",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/cornflower").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    WitherRose {
        identifier "minecraft:wither_rose",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/wither_rose").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    LilOfTheValley {
        identifier "minecraft:lily_of_the_valley",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/lily_of_the_valley").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    BrownMushroom {
        identifier "minecraft:brown_mushroom",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/brown_mushroom").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    RedMushroom {
        identifier "minecraft:red_mushroom",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/red_mushroom").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        }
                ]
            }
        },
        transparent true,
    }
    GoldBlock {
        identifier "minecraft:gold_block",
        props {},
        model BlockModel::square_block(["block/gold_block"; 6]),
    }
    IronBlock {
        identifier "minecraft:iron_block",
        props {},
        model BlockModel::square_block(["block/iron_block"; 6]),
    }
    Bricks {
        identifier "minecraft:bricks",
        props {},
        model BlockModel::square_block(["block/bricks"; 6]),
    }
    TNT {
        identifier "minecraft:tnt",
        props {
            unstable: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["block/tnt_top", "block/tnt_bottom", "block/tnt_side", "block/tnt_side", "block/tnt_side", "block/tnt_side"]),
    }
    Bookshelf {
        identifier "minecraft:bookshelf",
        props {},
        model BlockModel::square_block(["block/oak_planks", "block/oak_planks", "block/bookshelf", "block/bookshelf", "block/bookshelf", "block/bookshelf"]),
    }
    MossyCobblestone {
        identifier "minecraft:mossy_cobblestone",
        props {},
        model BlockModel::square_block(["block/bricks"; 6]),
    }
    Obsidian {
        identifier "minecraft:obsidian",
        props {},
        model BlockModel::square_block(["block/obsidian"; 6]),
    }
    Torch {
        identifier "minecraft:torch",
        props {},
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/torch").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.5625),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.5625, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Front,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Back,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
        light_color [255; 3],
        light_intensity 14,
    }
    WallTorch {
        identifier "minecraft:wall_torch",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East),
            ],
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/torch").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Right,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.5625),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Left,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.5625, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Front,
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            normal: ViewableDirectionBitMap::Back,
                            edge: false,
                        }
                ]
            }
        },
        collidable false,
        full false,
        light_color [109, 8, 100],
        light_intensity 14,
    }
    Fire {
        identifier "minecraft:fire",
        props {
            age: u8 = vec![
                PropertyType::UnsignedByte(0),
                PropertyType::UnsignedByte(1),
                PropertyType::UnsignedByte(2),
                PropertyType::UnsignedByte(3),
                PropertyType::UnsignedByte(4),
                PropertyType::UnsignedByte(5),
                PropertyType::UnsignedByte(6),
                PropertyType::UnsignedByte(7),
                PropertyType::UnsignedByte(8),
                PropertyType::UnsignedByte(9),
                PropertyType::UnsignedByte(10),
                PropertyType::UnsignedByte(11),
                PropertyType::UnsignedByte(12),
                PropertyType::UnsignedByte(13),
                PropertyType::UnsignedByte(14),
                PropertyType::UnsignedByte(15),
            ],
            east: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            north: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            south: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            up: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            west: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model BlockModel::square_block(["block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0"]),
        collidable false,
        full false,
        light_color [255; 3],
        light_intensity 14,
    }
    Spawner {
        identifier "minecraft:spawner",
        props {},
        model BlockModel::square_block(["block/spawner"; 6]),
    }
    OakStairs {
        identifier "minecraft:oak_stairs",
        props {
            facing: Direction = vec![
                PropertyType::Direction(Direction::North),
                PropertyType::Direction(Direction::South),
                PropertyType::Direction(Direction::West),
                PropertyType::Direction(Direction::East)
            ],
            top: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
            shape: StairShape = vec![
                PropertyType::StairShape(StairShape::Straight),
                PropertyType::StairShape(StairShape::InnerLeft),
                PropertyType::StairShape(StairShape::InnerRight),
                PropertyType::StairShape(StairShape::OuterLeft),
                PropertyType::StairShape(StairShape::OuterRight),
            ],
            waterlogged: bool = vec![
                PropertyType::Boolean(true),
                PropertyType::Boolean(false),
            ],
        },
        model {
            let lookup = AtlasIndex::new_lookup("block/oak_planks");

            BlockModel {
                faces: vec![
                    // Back faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        scale: Vector3::new(0.5, 1.0, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        normal: ViewableDirectionBitMap::Back,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.0, 1.0),
                        scale: Vector3::new(0.5, 0.5, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::BottomRight),
                        normal: ViewableDirectionBitMap::Back,
                        edge: true,
                    },

                    // Front faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.5, 1.0, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        normal: ViewableDirectionBitMap::Front,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.0, 0.0),
                        scale: Vector3::new(0.5, 0.5, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::BottomRight),
                        normal: ViewableDirectionBitMap::Front,
                        edge: true,
                    },

                    // Top faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 1.0, 0.0),
                        scale: Vector3::new(0.5, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        normal: ViewableDirectionBitMap::Top,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.5, 0.0),
                        scale: Vector3::new(0.5, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Right),
                        normal: ViewableDirectionBitMap::Top,
                        edge: false,
                    },

                    // Right faces
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.5, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Top),
                        normal: ViewableDirectionBitMap::Right,
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.5, 0.0),
                        scale: Vector3::new(0.0, 0.5, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Bottom),
                        normal: ViewableDirectionBitMap::Right,
                        edge: false,
                    },

                    // Left face
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 1.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Full),
                        normal: ViewableDirectionBitMap::Left,
                        edge: true,
                    },

                    // Bottom face
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Full),
                        normal: ViewableDirectionBitMap::Bottom,
                        edge: true,
                    },
                ]
            }
        },

    }
}
