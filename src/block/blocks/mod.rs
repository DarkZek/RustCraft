use crate::block::blocks::model::{BlockFace, BlockModel, Rotate};
use crate::block::Block;
use crate::game::physics::collider::BoxCollider;
use crate::helpers::{AtlasIndex, TextureSubdivisionMethod};
use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use crate::services::asset_service::index::TextureAtlasIndex;
use crate::services::asset_service::AssetService;
use crate::services::chunk_service::mesh::ViewableDirectionBitMap;
use crate::services::settings_service::SettingsService;
use fnv::FnvBuildHasher;
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
    properties: BlockType,
}

impl BlockStates {
    pub fn get_block(&self, id: usize) -> Option<Block> {
        match self.states.get(id) {
            None => None,
            Some(block_state) => Some(Block {
                block_type: &block_state.properties,
                block_state_index: id,
            }),
        }
    }

    pub fn generate(_asset: &AssetService, settings: &SettingsService) {
        let states = get_states();
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

            let mut states_path = settings.path.clone();
            states_path.push("debug_states");
            states_path.set_extension("txt");

            if let Result::Err(error) = std::fs::write(states_path, str) {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ChestType {
    Single,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum RedstoneWireDirection {
    Up,
    Side,
    None,
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
impl Default for ChestType {
    fn default() -> Self {
        ChestType::Single
    }
}
impl Default for RedstoneWireDirection {
    fn default() -> Self {
        RedstoneWireDirection::None
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! define_blocks {
    (
        $(
            $name:ident {
                $(i $index:expr,)?
                $(identifier $identifierfunc:expr,)?
                props {
                    $(
                        $fname:ident: $ftype:ty$( = $ignored:expr)?,
                    )*
                },
                $(states $states:expr,)?
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

        pub fn get_states() -> Vec<BlockStateEntry> {

            let blocks = vec![
                $(
                    $(
                        $states,
                    )?
                )+
            ];

            let mut states = Vec::new();

            // Flatten array
            let mut i = 0;
            for block in blocks {
                for state in block {
                    states.push(BlockStateEntry {
                        block_number: i,
                        properties: state
                    });
                }
                i += 1;
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
                                    min: Point { coords: Vector3::new(0.0, 0.0, 0.0) },
                                    max: Point { coords: Vector3::new(1.0, 1.0, 1.0) },
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
            pub fn get_model(&self, atlas_lookups: &HashMap<String, TextureAtlasIndex, FnvBuildHasher>) -> BlockModel {
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
        i 0,
        identifier "minecraft:air",
        props {},
        states {
            vec![BlockType::Air {}]
        },
        collidable false,
        full false,
    }
    Stone {
        i 1,
        identifier "minecraft:stone",
        props {},
        states {
            vec![BlockType::Stone {}]
        },
        model BlockModel::square_block(["block/stone"; 6]),
    }
    Granite {
        i 2,
        identifier "minecraft:granite",
        props {},
        states {
            vec![BlockType::Granite {}]
        },
        model BlockModel::square_block(["block/granite"; 6]),
    }
    SmoothGranite {
        i 3,
        identifier "minecraft:smooth_granite",
        props {},
        states {
            vec![BlockType::SmoothGranite {}]
        },
        model BlockModel::square_block(["block/polished_granite"; 6]),
    }
    Diorite {
        i 4,
        identifier "minecraft:diorite",
        props {},
        states {
            vec![BlockType::Diorite {}]
        },
        model BlockModel::square_block(["block/diorite"; 6]),
    }
    SmoothDiorite {
        i 5,
        identifier "minecraft:smooth_diorite",
        props {},
        states {
            vec![BlockType::SmoothDiorite {}]
        },
        model BlockModel::square_block(["block/polished_diorite"; 6]),
    }
    Andesite {
        i 6,
        identifier "minecraft:andesite",
        props {},
        states {
            vec![BlockType::Andesite {}]
        },
        model BlockModel::square_block(["block/andesite"; 6]),
    }
    SmoothAndesite {
        i 7,
        identifier "minecraft:smooth_andesite",
        props {},
        states {
            vec![BlockType::SmoothAndesite {}]
        },
        model BlockModel::square_block(["block/polished_andesite"; 6]),
    }
    GrassBlock {
        i 8,
        identifier "minecraft:grass_block",
        props {
            snowy: bool,
        },
        states {
            vec![
                BlockType::GrassBlock { snowy: true },
                BlockType::GrassBlock { snowy: false }
            ]
        },
        model {
            if snowy {
                BlockModel::square_block(["block/snow", "block/dirt", "block/grass_block_snow", "block/grass_block_snow", "block/grass_block_snow", "block/grass_block_snow"])
            } else {
                let mut faces = Vec::new();

                let grass_block_top = AtlasIndex::new_lookup("block/grass_block_top");
                let dirt = AtlasIndex::new_lookup("block/dirt");
                let grass_block_side = AtlasIndex::new_lookup("block/grass_block_side");

                // Top face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 1.0, 0.0),
                    scale: Vector3::new(1.0, 0.0, 1.0),
                    texture: grass_block_top.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Top,
                    color: [135, 255, 105, 255],
                    edge: true,
                });

                // Bottom face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    scale: Vector3::new(1.0, 0.0, 1.0),
                    texture: dirt.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Bottom,
                    color: [255; 4],
                    edge: true,
                });

                // Left face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    scale: Vector3::new(0.0, 1.0, 1.0),
                    texture: grass_block_side.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Left,
                    color: [255; 4],
                    edge: true,
                });

                // Right face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(1.0, 0.0, 0.0),
                    scale: Vector3::new(0.0, 1.0, 1.0),
                    texture: grass_block_side.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Right,
                    color: [255; 4],
                    edge: true,
                });

                // Front face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 0.0),
                    scale: Vector3::new(1.0, 1.0, 0.0),
                    texture: grass_block_side.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Front,
                    color: [255; 4],
                    edge: true,
                });

                // Back face
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 1.0),
                    scale: Vector3::new(1.0, 1.0, 0.0),
                    texture: grass_block_side.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Back,
                    color: [255; 4],
                    edge: true,
                });

                BlockModel { faces }
            }
        },
    }
    Dirt {
        i 9,
        identifier "minecraft:dirt",
        props {},
        states {
            vec![BlockType::Dirt {}]
        },
        model BlockModel::square_block(["block/dirt"; 6]),
    }
    CoarseDirt {
        i 10,
        identifier "minecraft:coarse_dirt",
        props {},
        states {
            vec![BlockType::CoarseDirt {}]
        },
        model BlockModel::square_block(["block/coarse_dirt"; 6]),
    }
    Podzol {
        i 11,
        identifier "minecraft:podzol",
        props {
            snowy: bool,
        },
        states {
            vec![
                BlockType::Podzol { snowy: true },
                BlockType::Podzol { snowy: false },
            ]
        },
        model BlockModel::square_block(["block/podzol_top", "block/dirt", "block/podzol_side", "block/podzol_side", "block/podzol_side", "block/podzol_side"]),
    }
    Cobblestone {
        i 12,
        identifier "minecraft:cobblestone",
        props {},
        states {
            vec![BlockType::Cobblestone {}]
        },
        model BlockModel::square_block(["block/cobblestone"; 6]),
    }
    Planks {
        i 13,
        identifier "minecraft:wooden_planks",
        props {
            variant: TreeVariant,
        },
        states {
            vec![
                BlockType::Planks { variant: TreeVariant::Oak },
                BlockType::Planks { variant: TreeVariant::Spruce },
                BlockType::Planks { variant: TreeVariant::Birch },
                BlockType::Planks { variant: TreeVariant::Jungle },
                BlockType::Planks { variant: TreeVariant::Acacia },
                BlockType::Planks { variant: TreeVariant::DarkOak }
            ]
        },
        model BlockModel::square_block(["block/oak_planks"; 6]),
    }
    OakSapling {
        i 14,
        identifier "minecraft:oak_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::OakSapling { stage: 0 },
                BlockType::OakSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/oak_sapling"),
        collidable false,
        full false,
    }
    SpruceSapling {
        i 15,
        identifier "minecraft:spruce_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::SpruceSapling { stage: 0 },
                BlockType::SpruceSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/spruce_sapling"),
        collidable false,
        full false,
    }
    BirchSapling {
        i 16,
        identifier "minecraft:birch_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::BirchSapling { stage: 0 },
                BlockType::BirchSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/birch_sapling"),
        collidable false,
        full false,
    }
    JungleSapling {
        i 17,
        identifier "minecraft:jungle_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::JungleSapling { stage: 0 },
                BlockType::JungleSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/jungle_sapling"),
        collidable false,
        full false,
    }
    AcaciaSapling {
        i 18,
        identifier "minecraft:acacia_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::AcaciaSapling { stage: 0 },
                BlockType::AcaciaSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/acacia_sapling"),
        collidable false,
        full false,
    }
    DarkOakSapling {
        i 19,
        identifier "minecraft:dark_oak_sapling",
        props {
            stage: u8,
        },
        states {
            vec![
                BlockType::DarkOakSapling { stage: 0 },
                BlockType::DarkOakSapling { stage: 1 }
            ]
        },
        model BlockModel::plant_block("block/dark_oak_sapling"),
        collidable false,
        full false,
    }
    Bedrock {
        i 20,
        identifier "minecraft:bedrock",
        props {},
        states {
            vec![
                BlockType::Bedrock { },
            ]
        },
        model BlockModel::square_block(["block/bedrock"; 6]),
    }
    Water {
        i 21,
        identifier "minecraft:water",
        props {
            water: u8,
        },
        states {
            vec![
                BlockType::Water { water: 0 },
                BlockType::Water { water: 1 },
                BlockType::Water { water: 2 },
                BlockType::Water { water: 3 },
                BlockType::Water { water: 4 },
                BlockType::Water { water: 5 },
                BlockType::Water { water: 6 },
                BlockType::Water { water: 7 },
                BlockType::Water { water: 8 },
                BlockType::Water { water: 9 },
                BlockType::Water { water: 10 },
                BlockType::Water { water: 11 },
                BlockType::Water { water: 12 },
                BlockType::Water { water: 13 },
                BlockType::Water { water: 14 },
                BlockType::Water { water: 15 },
            ]
        },
        model {
            let mut lookup = AtlasIndex::new_lookup("block/water_still").lookup;

            // Sprite is in 32 parts, select first part
            let single_sprite = lookup.height() / 32.0;
            lookup.v_max = lookup.v_min + single_sprite;

            BlockModel {
                faces: vec![
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.85, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Top,
                        color: [39, 90, 194, 255],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Bottom,
                        color: [39, 90, 194, 255],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [39, 90, 194, 255],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [39, 90, 194, 255],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Front,
                        color: [39, 90, 194, 255],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Back,
                        color: [39, 90, 194, 255],
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
        i 22,
        identifier "minecraft:lava",
        props {
            water: u8,
        },
        states {
            vec![
                BlockType::Water { water: 0 },
                BlockType::Water { water: 1 },
                BlockType::Water { water: 2 },
                BlockType::Water { water: 3 },
                BlockType::Water { water: 4 },
                BlockType::Water { water: 5 },
                BlockType::Water { water: 6 },
                BlockType::Water { water: 7 },
                BlockType::Water { water: 8 },
                BlockType::Water { water: 9 },
                BlockType::Water { water: 10 },
                BlockType::Water { water: 11 },
                BlockType::Water { water: 12 },
                BlockType::Water { water: 13 },
                BlockType::Water { water: 14 },
                BlockType::Water { water: 15 },
            ]
        },
        model BlockModel::square_block(["block/lava_still"; 6]),
        collidable false,
        transparent true,
    }
    Sand {
        i 23,
        identifier "minecraft:sand",
        props {},
        states {
            vec![BlockType::Sand { }]
        },
        model BlockModel::square_block(["block/sand"; 6]),
    }
    RedSand {
        i 24,
        identifier "minecraft:red_sand",
        props {},
        states {
            vec![BlockType::RedSand { }]
        },
        model BlockModel::square_block(["block/red_sand"; 6]),
    }
    Gravel {
        i 25,
        identifier "minecraft:gravel",
        props {},
        states {
            vec![BlockType::Gravel { }]
        },
        model BlockModel::square_block(["block/gravel"; 6]),
    }
    GoldOre {
        i 26,
        identifier "minecraft:gold_ore",
        props {},
        states {
            vec![BlockType::GoldOre { }]
        },
        model BlockModel::square_block(["block/gold_ore"; 6]),
    }
    IronOre {
        i 27,
        identifier "minecraft:iron_ore",
        props {},
        states {
            vec![BlockType::IronOre { }]
        },
        model BlockModel::square_block(["block/iron_ore"; 6]),
    }
    CoalOre {
        i 28,
        identifier "minecraft:coal_ore",
        props {},
        states {
            vec![BlockType::CoalOre { }]
        },
        model BlockModel::square_block(["block/coal_ore"; 6]),
    }
    OakLog {
        i 29,
        identifier "minecraft:oak_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::OakLog { axis: Axis::X },
                BlockType::OakLog { axis: Axis::Y },
                BlockType::OakLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/oak_log_top", "block/oak_log_top", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log"]),
    }
    SpruceLog {
        i 30,
        identifier "minecraft:spruce_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::SpruceLog { axis: Axis::X },
                BlockType::SpruceLog { axis: Axis::Y },
                BlockType::SpruceLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/spruce_log_top", "block/spruce_log_top", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log"]),
    }
    BirchLog {
        i 31,
        identifier "minecraft:birch_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::BirchLog { axis: Axis::X },
                BlockType::BirchLog { axis: Axis::Y },
                BlockType::BirchLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/birch_log_top", "block/birch_log_top", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log"]),
    }
    JungleLog {
        i 32,
        identifier "minecraft:jungle_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::JungleLog { axis: Axis::X },
                BlockType::JungleLog { axis: Axis::Y },
                BlockType::JungleLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/jungle_log_top", "block/jungle_log_top", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log"]),
    }
    AcaciaLog {
        i 33,
        identifier "minecraft:acacia_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::AcaciaLog { axis: Axis::X },
                BlockType::AcaciaLog { axis: Axis::Y },
                BlockType::AcaciaLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/acacia_log_top", "block/acacia_log_top", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log"]),
    }
    DarkOakLog {
        i 34,
        identifier "minecraft:dark_oak_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::DarkOakLog { axis: Axis::X },
                BlockType::DarkOakLog { axis: Axis::Y },
                BlockType::DarkOakLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/dark_oak_log_top", "block/dark_oak_log_top", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log"]),
    }
    StrippedSpruceLog {
        i 35,
        identifier "minecraft:stripped_spruce_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedSpruceLog { axis: Axis::X },
                BlockType::StrippedSpruceLog { axis: Axis::Y },
                BlockType::StrippedSpruceLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_spruce_log_top", "block/stripped_spruce_log_top", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log"]),
    }
    StrippedBirchLog {
        i 36,
        identifier "minecraft:stripped_birch_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedBirchLog { axis: Axis::X },
                BlockType::StrippedBirchLog { axis: Axis::Y },
                BlockType::StrippedBirchLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_birch_log_top", "block/stripped_birch_log_top", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log"]),
    }
    StrippedJungleLog {
        i 37,
        identifier "minecraft:stripped_jungle_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedJungleLog { axis: Axis::X },
                BlockType::StrippedJungleLog { axis: Axis::Y },
                BlockType::StrippedJungleLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_jungle_log_top", "block/stripped_jungle_log_top", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log"]),
    }
    StrippedAcaciaLog {
        i 38,
        identifier "minecraft:stripped_acacia_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedAcaciaLog { axis: Axis::X },
                BlockType::StrippedAcaciaLog { axis: Axis::Y },
                BlockType::StrippedAcaciaLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_acacia_log_top", "block/stripped_acacia_log_top", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log"]),
    }
    StrippedDarkOakLog {
        i 39,
        identifier "minecraft:stripped_dark_oak_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedDarkOakLog { axis: Axis::X },
                BlockType::StrippedDarkOakLog { axis: Axis::Y },
                BlockType::StrippedDarkOakLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_dark_oak_log_top", "block/stripped_dark_oak_log_top", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log"]),
    }
    StrippedOakLog {
        i 40,
        identifier "minecraft:stripped_oak_log",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedOakLog { axis: Axis::X },
                BlockType::StrippedOakLog { axis: Axis::Y },
                BlockType::StrippedOakLog { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_oak_log_top", "block/stripped_oak_log_top", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log"]),
    }
    OakWood {
        i 41,
        identifier "minecraft:oak_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::OakWood { axis: Axis::X },
                BlockType::OakWood { axis: Axis::Y },
                BlockType::OakWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log", "block/oak_log"]),
    }
    SpruceWood {
        i 42,
        identifier "minecraft:spruce_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::SpruceWood { axis: Axis::X },
                BlockType::SpruceWood { axis: Axis::Y },
                BlockType::SpruceWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log", "block/spruce_log"]),
    }
    BirchWood {
        i 43,
        identifier "minecraft:birch_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::BirchWood { axis: Axis::X },
                BlockType::BirchWood { axis: Axis::Y },
                BlockType::BirchWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log", "block/birch_log"]),
    }
    JungleWood {
        i 44,
        identifier "minecraft:jungle_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::JungleWood { axis: Axis::X },
                BlockType::JungleWood { axis: Axis::Y },
                BlockType::JungleWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log", "block/jungle_log"]),
    }
    AcaciaWood {
        i 45,
        identifier "minecraft:acacia_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::AcaciaWood { axis: Axis::X },
                BlockType::AcaciaWood { axis: Axis::Y },
                BlockType::AcaciaWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log", "block/acacia_log"]),
    }
    DarkOakWood {
        i 46,
        identifier "minecraft:jungle_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::DarkOakWood { axis: Axis::X },
                BlockType::DarkOakWood { axis: Axis::Y },
                BlockType::DarkOakWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log", "block/dark_oak_log"]),
    }
    StrippedOakWood {
        i 47,
        identifier "minecraft:stripped_oak_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedOakWood { axis: Axis::X },
                BlockType::StrippedOakWood { axis: Axis::Y },
                BlockType::StrippedOakWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log", "block/stripped_oak_log"]),
    }
    StrippedSpruceWood {
        i 48,
        identifier "minecraft:stripped_spruce_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedSpruceWood { axis: Axis::X },
                BlockType::StrippedSpruceWood { axis: Axis::Y },
                BlockType::StrippedSpruceWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log", "block/stripped_spruce_log"]),
    }
    StrippedBirchWood {
        i 49,
        identifier "minecraft:stripped_birch_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedBirchWood { axis: Axis::X },
                BlockType::StrippedBirchWood { axis: Axis::Y },
                BlockType::StrippedBirchWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log", "block/stripped_birch_log"]),
    }
    StrippedJungleWood {
        i 50,
        identifier "minecraft:stripped_jungle_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedJungleWood { axis: Axis::X },
                BlockType::StrippedJungleWood { axis: Axis::Y },
                BlockType::StrippedJungleWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log", "block/stripped_jungle_log"]),
    }
    StrippedAcaciaWood {
        i 51,
        identifier "minecraft:stripped_acacia_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedAcaciaWood { axis: Axis::X },
                BlockType::StrippedAcaciaWood { axis: Axis::Y },
                BlockType::StrippedAcaciaWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log", "block/stripped_acacia_log"]),
    }
    StrippedDarkOakWood {
        i 52,
        identifier "minecraft:stripped_dark_oak_wood",
        props {
            axis: Axis,
        },
        states {
            vec![
                BlockType::StrippedDarkOakWood { axis: Axis::X },
                BlockType::StrippedDarkOakWood { axis: Axis::Y },
                BlockType::StrippedDarkOakWood { axis: Axis::Z }
            ]
        },
        model BlockModel::square_block(["block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log", "block/stripped_dark_oak_log"]),
    }
    OakLeaves {
        i 53,
        identifier "minecraft:oak_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::OakLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_coloured_block(["block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves", "block/oak_leaves"], [60, 180, 35, 255]),
        transparent true,
    }
    SpruceLeaves {
        i 54,
        identifier "minecraft:spruce_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::SpruceLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_block(["block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves", "block/spruce_leaves"]),
        transparent true,
    }
    BirchLeaves {
        i 55,
        identifier "minecraft:birch_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::BirchLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_coloured_block(["block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves", "block/birch_leaves"], [130, 180, 95, 255]),
        transparent true,
    }
    JungleLeaves {
        i 56,
        identifier "minecraft:jungle_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::JungleLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_block(["block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves", "block/jungle_leaves"]),
        transparent true,
    }
    AcaciaLeaves {
        i 57,
        identifier "minecraft:acacia_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::AcaciaLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_block(["block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves", "block/acacia_leaves"]),
        transparent true,
    }
    DarkOakLeaves {
        i 58,
        identifier "minecraft:dark_oak_leaves",
        props {
            distance: u8,
            persistent: bool,
        },
        states {
            let mut states = Vec::new();
            for distance in 1..=7 {
                for persistent in [true, false] {
                    states.push(BlockType::DarkOakLeaves { distance, persistent })
                }
            }
            states
        },
        model BlockModel::square_coloured_block(["block/dark_oak_leaves"; 6], [60, 180, 35, 255]),
        transparent true,
    }
    Sponge {
        i 59,
        identifier "minecraft:sponge",
        props {},
        states {
            vec![BlockType::Sponge { }]
        },
        model BlockModel::square_block(["block/sponge"; 6]),
    }
    WetSponge {
        i 60,
        identifier "minecraft:wet_sponge",
        props {},
        states {
            vec![BlockType::WetSponge { }]
        },
        model BlockModel::square_block(["block/wet_sponge"; 6]),
    }
    Glass {
        i 61,
        identifier "minecraft:glass",
        props {},
        states {
            vec![BlockType::Glass { }]
        },
        model BlockModel::square_block(["block/glass"; 6]),
        transparent true,
    }
    LapisOre {
        i 62,
        identifier "minecraft:lapis_ore",
        props {},
        states {
            vec![BlockType::LapisOre { }]
        },
        model BlockModel::square_block(["block/lapis_ore"; 6]),
    }
    LapisBlock {
        i 63,
        identifier "minecraft:lapis_block",
        props {},
        states {
            vec![BlockType::LapisBlock { }]
        },
        model BlockModel::square_block(["block/lapis_block"; 6]),
    }
    Dispenser {
        i 64,
        identifier "minecraft:dispenser",
        props {
            facing: Direction,
            triggered: bool,
        },
        states {
            let directions = [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down];
            let mut states = Vec::new();
            for direction in directions {
                for triggered in [true, false] {
                    states.push(BlockType::Dispenser { facing: direction, triggered })
                }
            }
            states
        },
        model BlockModel::square_block(["block/furnace_top","block/furnace_top","block/furnace_top","block/furnace_top","block/dispenser_front","block/furnace_top"]),
    }
    Sandstone {
        i 65,
        identifier "minecraft:sandstone",
        props {},
        states {
            vec![BlockType::Sandstone { }]
        },
        model BlockModel::square_block(["block/sandstone_top", "block/sandstone_top", "block/sandstone", "block/sandstone", "block/sandstone", "block/sandstone"]),
    }
    ChiseledSandstone {
        i 66,
        identifier "minecraft:chiseled_sandstone",
        props {},
        states {
            vec![BlockType::ChiseledSandstone { }]
        },
        model BlockModel::square_block(["block/chiseled_sandstone"; 6]),
    }
    CutSandstone {
        i 67,
        identifier "minecraft:cut_sandstone",
        props {},
        states {
            vec![BlockType::CutSandstone { }]
        },
        model BlockModel::square_block(["block/cut_sandstone"; 6]),
    }
    NoteBlock {
        i 68,
        identifier "minecraft:note_block",
        props {
            instrument: Instrument,
            note: u8,
            powered: bool,
        },
        states {
            let instruments = [Instrument::Harp, Instrument::Basedrum, Instrument::Snare, Instrument::Hat, Instrument::Bass, Instrument::Flute,
                Instrument::Bell, Instrument::Guitar, Instrument::Chime, Instrument::Xylophone, Instrument::IronXylophone, Instrument::CowBell,
                Instrument::Didgeridoo, Instrument::Bit, Instrument::Banjo, Instrument::Pling];

            let notes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];

            let mut states = Vec::new();
            for instrument in instruments {
                for note in notes {
                    for powered in [true, false] {
                        states.push(BlockType::NoteBlock { instrument, note, powered })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["block/note_block"; 6]),
    }
    WhiteBed {
        i 69,
        identifier "minecraft:white_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::WhiteBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    OrangeBed {
        i 70,
        identifier "minecraft:orange_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::OrangeBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    MagentaBed {
        i 71,
        identifier "minecraft:magenta_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::MagentaBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LightBlueBed {
        i 72,
        identifier "minecraft:light_blue_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::LightBlueBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    YellowBed {
        i 73,
        identifier "minecraft:yellow_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::YellowBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LimeBed {
        i 74,
        identifier "minecraft:lime_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::LimeBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PinkBed {
        i 75,
        identifier "minecraft:pink_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::PinkBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    GrayBed {
        i 76,
        identifier "minecraft:gray_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::GrayBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    LightGrayBed {
        i 77,
        identifier "minecraft:light_gray_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::LightGrayBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    CyanBed {
        i 78,
        identifier "minecraft:cyan_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::CyanBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PurpleBed {
        i 79,
        identifier "minecraft:purple_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::PurpleBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BlueBed {
        i 80,
        identifier "minecraft:blue_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::BlueBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BrownBed {
        i 81,
        identifier "minecraft:brown_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::BrownBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    GreenBed {
        i 82,
        identifier "minecraft:green_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::GreenBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    RedBed {
        i 83,
        identifier "minecraft:red_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::RedBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    BlackBed {
        i 84,
        identifier "minecraft:black_bed",
        props {
            facing: Direction,
            occupied: bool,
            head_part: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];

            let mut states = Vec::new();
            for facing in facings {
                for occupied in [true, false] {
                    for head_part in [true, false] {
                        states.push(BlockType::BlackBed { facing, occupied, head_part })
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["mcv3/error"; 6]),
    }
    PoweredRail {
        i 85,
        identifier "minecraft:powered_rail",
        props {
            powered: bool,
            shape: RailShape,
        },
        states {
            let shapes = [RailShape::NorthSouth, RailShape::EastWest, RailShape::AscendingEast, RailShape::AscendingWest, RailShape::AscendingNorth, RailShape::AscendingSouth];

            let mut states = Vec::new();
            for powered in [true, false] {
                for shape in shapes {
                    states.push(BlockType::PoweredRail { powered, shape })
                }
            }
            states
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/powered_rail").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Top,
                            color: [255; 4],
                            edge: false,
                        }
                    ],
            }
        },
        collidable false,
        full false,
    }
    DetectorRail {
        i 86,
        identifier "minecraft:detector_rail",
        props {
            powered: bool,
            shape: RailShape,
        },
        states {
            let shapes = [RailShape::NorthSouth, RailShape::EastWest, RailShape::AscendingEast, RailShape::AscendingWest, RailShape::AscendingNorth, RailShape::AscendingSouth];

            let mut states = Vec::new();
            for powered in [true, false] {
                for shape in shapes {
                    states.push(BlockType::PoweredRail { powered, shape })
                }
            }
            states
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/detector_rail").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());
            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Top,
                            color: [255; 4],
                            edge: false,
                        }
                    ]
            }
        },
        collidable false,
        full false,
    }
    StickyPiston {
        i 87,
        identifier "minecraft:sticky_piston",
        props {
            extended: bool,
            facing: Direction,
        },
        states {
            let facings = [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down];

            let mut states = Vec::new();
            for extended in [true, false] {
                for facing in facings {
                    states.push(BlockType::StickyPiston { extended, facing })
                }
            }
            states
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top_sticky", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    Cobweb {
        i 88,
        identifier "minecraft:cobweb",
        props {},
        states {
            vec![BlockType::Cobweb { }]
        },
        model BlockModel::square_block(["block/cobweb"; 6]),
        transparent true,
    }
    Grass {
        i 89,
        identifier "minecraft:grass",
        props {},
        states {
            vec![BlockType::Grass { }]
        },
        model {
            let lookup = AtlasIndex::new_lookup("block/grass").lookup;

            BlockModel {
                faces: vec![
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [135, 255, 105, 255],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [135, 255, 105, 255],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [135, 255, 105, 255],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [135, 255, 105, 255],
                        edge: false,
                    },
                ],
            }
        },
        collidable false,
        full false,
        transparent true,
    }
    Fern {
        i 90,
        identifier "minecraft:fern",
        props {},
        states {
            vec![BlockType::Fern { }]
        },
        model BlockModel::plant_block("block/fern"),
        collidable false,
        transparent true,
    }
    DeadBush {
        i 91,
        identifier "minecraft:dead_bush",
        props {},
        states {
            vec![BlockType::DeadBush { }]
        },
        model BlockModel::plant_block("block/dead_bush"),
        transparent true,
    }
    Seagrass {
        i 92,
        identifier "minecraft:seagrass",
        props {},
        states {
            vec![BlockType::Seagrass { }]
        },
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
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 0.85, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: false,
                    },
                    // Water
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.85, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Top,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Bottom,
                    color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.85, 1.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Front,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        scale: Vector3::new(1.0, 0.85, 0.0),
                        texture: water_lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Back,
                        color: [255; 4],
                        edge: true,
                    },
                ]
            }
        },
        transparent true,
        waterlogged true,
    }
    TallSeagrass {
        i 93,
        identifier "minecraft:tall_seagrass",
        props {
            upper_half: bool,
        },
        states {
            vec![
                BlockType::TallSeagrass { upper_half: true },
                BlockType::TallSeagrass { upper_half: false }
            ]
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
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Left,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Right,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Left,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(-1.0, 1.0, 1.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Right,
                            color: [255; 4],
                            edge: false,
                        },
                        // Water
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.85, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Top,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.0, 1.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Bottom,
                            color: [255; 4],
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Left,
                            color: [255; 4],
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(1.0, 0.0, 0.0),
                            scale: Vector3::new(0.0, 0.85, 1.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Right,
                            color: [255; 4],
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 0.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Front,
                            color: [255; 4],
                            edge: true,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.0, 0.0, 1.0),
                            scale: Vector3::new(1.0, 0.85, 0.0),
                            texture: water_lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Back,
                            color: [255; 4],
                            edge: true,
                        },
                ],
            }
        },
        transparent true,
        waterlogged true,
    }
    Piston {
        i 94,
        identifier "minecraft:piston",
        props {
            extended: bool,
            facing: Direction,
        },
        states {
            let facings = [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down];

            let mut states = Vec::new();
            for extended in [true, false] {
                for facing in facings {
                    states.push(BlockType::StickyPiston { extended, facing })
                }
            }
            states
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    PistonHead {
        i 95,
        identifier "minecraft:piston_head",
        props {
            facing: Direction,
            short: bool,
            is_sticky: bool,
        },
        states {
            let facings = [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down];

            let mut states = Vec::new();
            for facing in facings {
                for short in [true, false] {
                    for is_sticky in [false, true] {
                        states.push(BlockType::PistonHead { facing, short, is_sticky })
                    }
                }
            }

            states
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
        collidable false,
        full false,
    }
    WhiteWool {
        i 96,
        identifier "minecraft:white_wool",
        props {},
        states {
            vec![BlockType::WhiteWool { }]
        },
        model BlockModel::square_block(["block/white_wool"; 6]),
    }
    OrangeWool {
        i 97,
        identifier "minecraft:orange_wool",
        props {},
        states {
            vec![BlockType::OrangeWool { }]
        },
        model BlockModel::square_block(["block/orange_wool"; 6]),
    }
    MagentaWool {
        i 98,
        identifier "minecraft:magenta_wool",
        props {},
        states {
            vec![BlockType::MagentaWool { }]
        },
        model BlockModel::square_block(["block/magenta_wool"; 6]),
    }
    LightBlueWool {
        i 99,
        identifier "minecraft:light_blue_wool",
        props {},
        states {
            vec![BlockType::LightBlueWool { }]
        },
        model BlockModel::square_block(["block/light_blue_wool"; 6]),
    }
    YellowWool {
        i 100,
        identifier "minecraft:yellow_wool",
        props {},
        states {
            vec![BlockType::YellowWool { }]
        },
        model BlockModel::square_block(["block/yellow_wool"; 6]),
    }
    LimeWool {
        i 101,
        identifier "minecraft:lime_wool",
        props {},
        states {
            vec![BlockType::LimeWool { }]
        },
        model BlockModel::square_block(["block/lime_wool"; 6]),
    }
    PinkWool {
        i 102,
        identifier "minecraft:pink_wool",
        props {},
        states {
            vec![BlockType::PinkWool { }]
        },
        model BlockModel::square_block(["block/pink_wool"; 6]),
    }
    GrayWool {
        i 103,
        identifier "minecraft:gray_wool",
        props {},
        states {
            vec![BlockType::GrayWool { }]
        },
        model BlockModel::square_block(["block/gray_wool"; 6]),
    }
    LightGrayWool {
        i 104,
        identifier "minecraft:light_gray_wool",
        props {},
        states {
            vec![BlockType::LightGrayWool { }]
        },
        model BlockModel::square_block(["block/light_gray_wool"; 6]),
    }
    CyanWool {
        i 105,
        identifier "minecraft:cyan_wool",
        props {},
        states {
            vec![BlockType::CyanWool { }]
        },
        model BlockModel::square_block(["block/cyan_wool"; 6]),
    }
    PurpleWool {
        i 106,
        identifier "minecraft:purple_wool",
        props {},
        states {
            vec![BlockType::PurpleWool { }]
        },
        model BlockModel::square_block(["block/purple_wool"; 6]),
    }
    BlueWool {
        i 107,
        identifier "minecraft:blue_wool",
        props {},
        states {
            vec![BlockType::BlueWool { }]
        },
        model BlockModel::square_block(["block/blue_wool"; 6]),
    }
    BrownWool {
        i 108,
        identifier "minecraft:brown_wool",
        props {},
        states {
            vec![BlockType::BrownWool { }]
        },
        model BlockModel::square_block(["block/brown_wool"; 6]),
    }
    GreenWool {
        i 109,
        identifier "minecraft:green_wool",
        props {},
        states {
            vec![BlockType::GreenWool { }]
        },
        model BlockModel::square_block(["block/green_wool"; 6]),
    }
    RedWool {
        i 110,
        identifier "minecraft:red_wool",
        props {},
        states {
            vec![BlockType::RedWool { }]
        },
        model BlockModel::square_block(["block/red_wool"; 6]),
    }
    BlackWool {
        i 111,
        identifier "minecraft:black_wool",
        props {},
        states {
            vec![BlockType::BlackWool { }]
        },
        model BlockModel::square_block(["block/black_wool"; 6]),
    }
    MovingPiston {
        i 112,
        identifier "minecraft:moving_piston",
        props {
            facing: Direction,
            is_sticky: bool,
        },
        states {
            let facings = [Direction::North, Direction::East, Direction::South, Direction::West, Direction::Up, Direction::Down];

            let mut states = Vec::new();
            for facing in facings {
                for is_sticky in [false, true] {
                    states.push(BlockType::MovingPiston { facing, is_sticky })
                }
            }

            states
        },
        model BlockModel::square_block(["block/piston_side", "block/piston_side", "block/piston_side", "block/piston_side", "block/piston_top", "block/piston_bottom"]),
    }
    Dandelion {
        i 113,
        identifier "minecraft:dandelion",
        props {},
        states {
            vec![BlockType::Dandelion { }]
        },
        model BlockModel::plant_block("block/dandelion"),
        transparent true,
    }
    Poppy {
        i 114,
        identifier "minecraft:poppy",
        props {},
        states {
            vec![BlockType::Poppy { }]
        },
        model BlockModel::plant_block("block/poppy"),
        transparent true,
    }
    BlueOrchid {
        i 115,
        identifier "minecraft:blue_orchid",
        props {},
        states {
            vec![BlockType::BlueOrchid { }]
        },
        model BlockModel::plant_block("block/blue_orchid"),
        transparent true,
    }
    Allium {
        i 116,
        identifier "minecraft:allium",
        props {},
        states {
            vec![BlockType::Allium { }]
        },
        model BlockModel::plant_block("block/allium"),
        transparent true,
    }
    AzureBluet {
        i 117,
        identifier "minecraft:azure_bluet",
        props {},
        states {
            vec![BlockType::AzureBluet { }]
        },
        model BlockModel::plant_block("block/azure_bluet"),
        transparent true,
    }
    RedTulip {
        i 118,
        identifier "minecraft:red_tulip",
        props {},
        states {
            vec![BlockType::RedTulip { }]
        },
        model BlockModel::plant_block("block/red_tulip"),
        transparent true,
    }
    OrangeTulip {
        i 119,
        identifier "minecraft:orange_tulip",
        props {},
        states {
            vec![BlockType::OrangeTulip { }]
        },
        model BlockModel::plant_block("block/orange_tulip"),
        transparent true,
    }
    WhiteTulip {
        i 120,
        identifier "minecraft:white_tulip",
        props {},
        states {
            vec![BlockType::WhiteTulip { }]
        },
        model BlockModel::plant_block("block/white_tulip"),
        transparent true,
    }
    PinkTulip {
        i 121,
        identifier "minecraft:pink_tulip",
        props {},
        states {
            vec![BlockType::PinkTulip { }]
        },
        model BlockModel::plant_block("block/pink_tulip"),
        transparent true,
    }
    OxeyeDaisy {
        i 122,
        identifier "minecraft:oxeye_daisy",
        props {},
        states {
            vec![BlockType::OxeyeDaisy { }]
        },
        model BlockModel::plant_block("block/oxeye_daisy"),
        transparent true,
    }
    Cornflower {
        i 123,
        identifier "minecraft:cornflower",
        props {},
        states {
            vec![BlockType::Cornflower { }]
        },
        model BlockModel::plant_block("block/cornflower"),
        transparent true,
    }
    WitherRose {
        i 124,
        identifier "minecraft:wither_rose",
        props {},
        states {
            vec![BlockType::WitherRose { }]
        },
        model BlockModel::plant_block("block/wither_rose"),
        transparent true,
    }
    LilyOfTheValley {
        i 125,
        identifier "minecraft:lily_of_the_valley",
        props {},
        states {
            vec![BlockType::LilyOfTheValley { }]
        },
        model BlockModel::plant_block("block/lily_of_the_valley"),
        transparent true,
    }
    BrownMushroom {
        i 126,
        identifier "minecraft:brown_mushroom",
        props {},
        states {
            vec![BlockType::BrownMushroom { }]
        },
        model BlockModel::plant_block("block/brown_mushroom"),
        transparent true,
    }
    RedMushroom {
        i 127,
        identifier "minecraft:red_mushroom",
        props {},
        states {
            vec![BlockType::RedMushroom { }]
        },
        model BlockModel::plant_block("block/red_mushroom"),
        transparent true,
    }
    GoldBlock {
        i 128,
        identifier "minecraft:gold_block",
        props {},
        states {
            vec![BlockType::GoldBlock { }]
        },
        model BlockModel::square_block(["block/gold_block"; 6]),
    }
    IronBlock {
        i 129,
        identifier "minecraft:iron_block",
        props {},
        states {
            vec![BlockType::IronBlock { }]
        },
        model BlockModel::square_block(["block/iron_block"; 6]),
    }
    Bricks {
        i 130,
        identifier "minecraft:bricks",
        props {},
        states {
            vec![BlockType::Bricks { }]
        },
        model BlockModel::square_block(["block/bricks"; 6]),
    }
    TNT {
        i 131,
        identifier "minecraft:tnt",
        props {
            unstable: bool,
        },
        states {
            vec![
                BlockType::TNT { unstable: true },
                BlockType::TNT { unstable: false }
            ]
        },
        model BlockModel::square_block(["block/tnt_top", "block/tnt_bottom", "block/tnt_side", "block/tnt_side", "block/tnt_side", "block/tnt_side"]),
    }
    Bookshelf {
        i 131,
        identifier "minecraft:bookshelf",
        props {},
        states {
            vec![BlockType::Bookshelf { }]
        },
        model BlockModel::square_block(["block/oak_planks", "block/oak_planks", "block/bookshelf", "block/bookshelf", "block/bookshelf", "block/bookshelf"]),
    }
    MossyCobblestone {
        i 132,
        identifier "minecraft:mossy_cobblestone",
        props {},
        states {
            vec![BlockType::MossyCobblestone { }]
        },
        model BlockModel::square_block(["block/bricks"; 6]),
    }
    Obsidian {
        i 133,
        identifier "minecraft:obsidian",
        props {},
        states {
            vec![BlockType::Obsidian { }]
        },
        model BlockModel::square_block(["block/obsidian"; 6]),
    }
    Torch {
        i 134,
        identifier "minecraft:torch",
        props {},
        states {
            vec![BlockType::Torch { }]
        },
        model {
            let lookup = AtlasIndex::new_lookup("block/torch").lookup;

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Right,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.5625),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Left,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.5625, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Front,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Back,
                            color: [255; 4],
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
        i 135,
        identifier "minecraft:wall_torch",
        props {
            facing: Direction,
        },
        states {
            vec![
                BlockType::WallTorch { facing: Direction::North },
                BlockType::WallTorch { facing: Direction::South },
                BlockType::WallTorch { facing: Direction::West },
                BlockType::WallTorch { facing: Direction::East }
            ]
        },
        model {
            let lookup = ATLAS_LOOKUPS.get().unwrap().get("block/torch").unwrap_or(ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap());

            BlockModel {
                faces: vec![
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Right,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.5625),
                            scale: Vector3::new(0.125, 0.625, 0.0),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Left,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.5625, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Front,
                            color: [255; 4],
                            edge: false,
                        },
                        BlockFace {
                            bottom_left: Vector3::new(0.4375, 0.0, 0.4375),
                            scale: Vector3::new(0.0, 0.625, 0.125),
                            texture: lookup.clone(),
                            texture_rotation: Rotate::Deg0,
                            normal: ViewableDirectionBitMap::Back,
                            color: [255; 4],
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
        i 136,
        identifier "minecraft:fire",
        props {
            age: u8,
            east: bool,
            north: bool,
            south: bool,
            up: bool,
            west: bool,
        },
        states {
            let mut states = Vec::new();
            for age in 0..=15 {
                for east in [true, false] {
                    for north in [true, false] {
                        for south in [true, false] {
                            for up in [true, false] {
                                for west in [true, false] {
                                    states.push(BlockType::Fire { age, east, north, south, up, west });
                                }
                            }
                        }
                    }
                }
            }
            states
        },
        model BlockModel::square_block(["block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0", "block/fire_0"]),
        collidable false,
        full false,
        light_color [255; 3],
        light_intensity 14,
    }
    Spawner {
        i 137,
        identifier "minecraft:spawner",
        props {},
        states {
            vec![BlockType::Spawner { }]
        },
        model BlockModel::square_block(["block/spawner"; 6]),
    }
    OakStairs {
        i 138,
        identifier "minecraft:oak_stairs",
        props {
            facing: Direction,
            top: bool,
            shape: StairShape,
            waterlogged: bool,
        },
        states {
            let facings = [Direction::North, Direction::South, Direction::West, Direction::East];
            let shapes = [StairShape::Straight, StairShape::InnerLeft, StairShape::InnerRight, StairShape::OuterLeft, StairShape::OuterRight];

            let mut states = Vec::new();
            for facing in facings {
                for top in [true, false] {
                    for shape in shapes {
                        for waterlogged in [true, false] {
                            states.push(BlockType::OakStairs { facing, top, shape, waterlogged });
                        }
                    }
                }
            }
            states
        },
        model {
            let lookup = AtlasIndex::new_lookup("block/oak_planks");
            let lookup = AtlasIndex::new_lookup("block/oak_planks");

            let mut model = BlockModel {
                faces: vec![
                    // Back faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 1.0),
                        scale: Vector3::new(0.5, 1.0, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Back,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.0, 1.0),
                        scale: Vector3::new(0.5, 0.5, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::BottomRight),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Back,
                        color: [255; 4],
                        edge: true,
                    },

                    // Front faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.5, 1.0, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Front,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.0, 0.0),
                        scale: Vector3::new(0.5, 0.5, 0.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::BottomRight),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Front,
                        color: [255; 4],
                        edge: true,
                    },

                    // Top faces
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 1.0, 0.0),
                        scale: Vector3::new(0.5, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Left),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Top,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.5, 0.0),
                        scale: Vector3::new(0.5, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Right),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Top,
                        color: [255; 4],
                        edge: false,
                    },

                    // Right faces ("front faces of the stair")
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 0.5, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Top),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: true,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.5, 0.5, 0.0),
                        scale: Vector3::new(0.0, 0.5, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Bottom),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: false,
                    },

                    // Left face
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(0.0, 1.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Full),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: true,
                    },

                    // Bottom face
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 0.0, 1.0),
                        texture: lookup.get_subdivision(TextureSubdivisionMethod::Full),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Bottom,
                        color: [255; 4],
                        edge: true,
                    },
                ]
            };

            if top {
                // Move face
                model.invert_y();
            }

            if facing == Direction::South {
                model.rotate_xz(crate::block::blocks::model::Rotate::Deg90);
            }

            if facing == Direction::North {
                model.rotate_xz(crate::block::blocks::model::Rotate::Deg270);
            }

            if facing == Direction::East {
                model.rotate_xz(crate::block::blocks::model::Rotate::Deg180);
            }

            model
        },
    }
    Chest {
        i 129,
        identifier "minecraft:chest",
        props {
            facing: Direction,
            chest_type: ChestType,
            waterlogged: bool,
        },
        states {
            let mut states = Vec::new();

            for facing in [Direction::North, Direction::East, Direction::South, Direction::West] {
                for chest_type in [ChestType::Single, ChestType::Left, ChestType::Right] {
                    for waterlogged in [false, true] {
                        states.push(BlockType::Chest { facing, chest_type, waterlogged })
                    }
                }
            }

            states
        },
        model BlockModel::square_block(["entity/chest/normal"; 6]),
        transparent true,
    }
    RedstoneWire {
        i 130,
        identifier "minecraft:redstone_wire",
        props {
            east: RedstoneWireDirection,
            north: RedstoneWireDirection,
            power: u8,
            south: RedstoneWireDirection,
            west: RedstoneWireDirection,
        },
        states {
            let mut states = Vec::new();

            for east in [RedstoneWireDirection::Up, RedstoneWireDirection::Side, RedstoneWireDirection::None] {
                for north in [RedstoneWireDirection::Up, RedstoneWireDirection::Side, RedstoneWireDirection::None] {
                    for power in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
                        for south in [RedstoneWireDirection::Up, RedstoneWireDirection::Side, RedstoneWireDirection::None] {
                            for west in [RedstoneWireDirection::Up, RedstoneWireDirection::Side, RedstoneWireDirection::None] {
                                states.push(BlockType::RedstoneWire { east, north, power, south, west })
                            }
                        }
                    }
                }
            }

            states
        },
        model {

            let redstone_dust_dot = AtlasIndex::new_lookup("block/redstone_dust_dot");
            let redstone_dust = AtlasIndex::new_lookup("block/redstone_dust_line0");

            let top_redstone_dust = redstone_dust.get_subdivision(TextureSubdivisionMethod::Top);
            let bottom_redstone_dust = redstone_dust.get_subdivision(TextureSubdivisionMethod::Bottom);

            let mut faces = vec![];

            // Directions on the bottom tile
            if north == RedstoneWireDirection::Side {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.05, 0.0),
                    scale: Vector3::new(1.0, 0.0, 0.5),
                    texture: bottom_redstone_dust,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Top,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if south == RedstoneWireDirection::Side {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.05, 0.5),
                    scale: Vector3::new(1.0, 0.0, 0.5),
                    texture: top_redstone_dust,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Top,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if west == RedstoneWireDirection::Side {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.05, 0.0),
                    scale: Vector3::new(0.5, 0.0, 1.0),
                    texture: top_redstone_dust,
                    texture_rotation: Rotate::Deg90,
                    normal: ViewableDirectionBitMap::Top,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if east == RedstoneWireDirection::Side {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.5, 0.05, 0.0),
                    scale: Vector3::new(0.5, 0.0, 1.0),
                    texture: bottom_redstone_dust,
                    texture_rotation: Rotate::Deg90,
                    normal: ViewableDirectionBitMap::Top,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }

            // Directions for climbing redstone
            if south == RedstoneWireDirection::Up {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 0.95),
                    scale: Vector3::new(1.0, 1.0, 0.0),
                    texture: redstone_dust.lookup,
                    normal: ViewableDirectionBitMap::Front,
                    texture_rotation: Rotate::Deg0,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if north == RedstoneWireDirection::Up {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.0, 0.05),
                    scale: Vector3::new(1.0, 1.0, 0.0),
                    texture: redstone_dust.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Back,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if west == RedstoneWireDirection::Up {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.05, 0.0, 0.0),
                    scale: Vector3::new(0.0, 1.0, 1.0),
                    texture: redstone_dust.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Right,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }
            if east == RedstoneWireDirection::Up {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.95, 0.0, 0.0),
                    scale: Vector3::new(0.0, 1.0, 1.0),
                    texture: redstone_dust.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Left,
                    color: [255, 0, 0, 255],
                    edge: false,
                })
            }

            // If no directions set then it's just a dot
            if faces.len() == 0 {
                faces.push(BlockFace {
                    bottom_left: Vector3::new(0.0, 0.05, 0.0),
                    scale: Vector3::new(1.0, 0.0, 1.0),
                    texture: redstone_dust_dot.lookup,
                    texture_rotation: Rotate::Deg0,
                    normal: ViewableDirectionBitMap::Top,
                    color: [255, 0, 0, 255],
                    edge: true,
                });
            }

            BlockModel { faces }
        },
        transparent true,
    }
    DiamondOre {
        i 131,
        identifier "minecraft:diamond_ore",
        props {},
        states {
            vec![BlockType::DiamondOre { }]
        },
        model BlockModel::square_block(["block/diamond_ore"; 6]),
    }
    DiamondBlock {
        i 132,
        identifier "minecraft:diamond_block",
        props {},
        states {
            vec![BlockType::DiamondBlock { }]
        },
        model BlockModel::square_block(["block/diamond_ore"; 6]),
    }
    CraftingTable {
        i 133,
        identifier "minecraft:crafting_table",
        props {},
        states {
            vec![BlockType::CraftingTable { }]
        },
        model BlockModel::square_block(["block/crafting_table_top", "block/oak_planks", "block/crafting_table_front", "block/crafting_table_side", "block/crafting_table_side", "block/crafting_table_side"]),
    }
    Wheat {
        i 134,
        identifier "minecraft:wheat",
        props { age: u8, },
        states {
            vec![
                BlockType::Wheat { age: 0 },
                BlockType::Wheat { age: 1 },
                BlockType::Wheat { age: 2 },
                BlockType::Wheat { age: 3 },
                BlockType::Wheat { age: 4 },
                BlockType::Wheat { age: 5 },
                BlockType::Wheat { age: 6 },
                BlockType::Wheat { age: 7 }
            ]
        },
        model {
            let lookup = AtlasIndex::new_lookup(&format!("block/wheat_stage{}", age)).lookup;

            BlockModel {
                faces: vec![
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(0.0, 0.0, 0.0),
                        scale: Vector3::new(1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Left,
                        color: [255; 4],
                        edge: false,
                    },
                    BlockFace {
                        bottom_left: Vector3::new(1.0, 0.0, 0.0),
                        scale: Vector3::new(-1.0, 1.0, 1.0),
                        texture: lookup.clone(),
                        texture_rotation: Rotate::Deg0,
                        normal: ViewableDirectionBitMap::Right,
                        color: [255; 4],
                        edge: false,
                    }
                ]
            }
        },
        transparent true,
    }
}
