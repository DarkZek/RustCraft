use nalgebra::Vector3;
use crate::aabb::Aabb;
use crate::atlas::{TEXTURE_ATLAS, TextureAtlasIndex};
use crate::block::face::Face;
use crate::block::types::Block;
use crate::block::blocks::{BlockImpl, BlockUid};
use crate::viewable_direction::ViewableDirectionBitMap;

pub struct PipeBlock;

impl BlockImpl for PipeBlock {
    const IDENTIFIER: &'static str = "mcv3::block::Pipe";

    fn get_variants() -> Vec<Block> {
        pipe()
            .into_iter()
            .map(|face| {
                Block {
                    identifier: "mcv3::block::Pipe".to_string(),
                    translucent: true,
                    full: true,
                    draw_betweens: false,
                    faces: face,
                    collision_boxes: vec![
                        Aabb::new(
                            Vector3::new(0.25, 0.25, 0.0),
                            Vector3::new(0.5, 0.5, 1.0),
                        )
                    ],
                    bounding_boxes: vec![
                        Aabb::new(
                            Vector3::new(0.25, 0.25, 0.0),
                            Vector3::new(0.5, 0.5, 1.0),
                        )
                    ],
                    emission: [0; 4],
                }
            })
            .collect::<Vec<Block>>()
    }

    fn parse_block_state(id: BlockUid) -> Self {
        Self
    }
}

pub fn pipe() -> Vec<Vec<Face>> {
    let texture = *TEXTURE_ATLAS.get().index.get("game/pipe_end").unwrap_or(&TextureAtlasIndex::default());

    let center_piece = vec![
        Face {
            top_left: Vector3::new(0.75, 0.25, 0.25),
            top_right: Vector3::new(0.75, 0.25, 0.75),
            bottom_left: Vector3::new(0.75, 0.75, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Top,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.25, 0.25, 0.25),
            top_right: Vector3::new(0.25, 0.75, 0.25),
            bottom_left: Vector3::new(0.25, 0.25, 0.75),
            texture,
            direction: ViewableDirectionBitMap::Bottom,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.25, 0.25, 0.25),
            top_right: Vector3::new(0.75, 0.25, 0.25),
            bottom_left: Vector3::new(0.25, 0.75, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Left,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.25, 0.75, 0.75),
            top_right: Vector3::new(0.75, 0.75, 0.75),
            bottom_left: Vector3::new(0.25, 0.25, 0.75),
            texture,
            direction: ViewableDirectionBitMap::Right,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.75, 0.25, 0.75),
            top_right: Vector3::new(0.75, 0.25, 0.25),
            bottom_left: Vector3::new(0.75, 0.75, 0.75),
            texture,
            direction: ViewableDirectionBitMap::Bottom,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.25, 0.25, 0.75),
            top_right: Vector3::new(0.25, 0.75, 0.75),
            bottom_left: Vector3::new(0.25, 0.25, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Top,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.75, 0.25, 0.25),
            top_right: Vector3::new(0.25, 0.25, 0.25),
            bottom_left: Vector3::new(0.75, 0.75, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Left,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.75, 0.75, 0.75),
            top_right: Vector3::new(0.25, 0.75, 0.75),
            bottom_left: Vector3::new(0.75, 0.25, 0.75),
            texture,
            direction: ViewableDirectionBitMap::Right,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.75, 0.75, 0.25),
            top_right: Vector3::new(0.75, 0.75, 0.75),
            bottom_left: Vector3::new(0.25, 0.75, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Top,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
        Face {
            top_left: Vector3::new(0.75, 0.75, 0.25),
            top_right: Vector3::new(0.75, 0.75, 0.75),
            bottom_left: Vector3::new(0.25, 0.75, 0.25),
            texture,
            direction: ViewableDirectionBitMap::Bottom,
            edge: false,
            normal: ViewableDirectionBitMap::Top.to_normal(),
            wind_strengths: None
        },
    ];

    let mut states = Vec::new();

    // Toggle each side
    for i in 0..(2_i32.pow(6)) {
        let sides_active = [
            i & 0b100000 > 0,
            i & 0b010000 > 0,
            i & 0b001000 > 0,
            i & 0b000100 > 0,
            i & 0b000010 > 0,
            i & 0b000001 > 0,
        ];

        let mut working_copy = center_piece.clone();

        for x in 0..6 {
            if sides_active[x] == false {
                continue;
            }

            working_copy
                .append(&mut data(texture).get_mut(x).unwrap().to_vec());
        }

        states.push(working_copy);
    }

    states
}

fn data<'a>(texture: TextureAtlasIndex) -> Vec<[Face; 8]> {
    vec![
        // Top
        [
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 1.0, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.25),
                top_right: Vector3::new(0.25, 1.0, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 1.0, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 1.0, 0.75),
                top_right: Vector3::new(0.75, 1.0, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 1.0, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 1.0, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.25, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 1.0, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 1.0, 0.75),
                top_right: Vector3::new(0.25, 1.0, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
        // Bottom
        [
            Face {
                top_left: Vector3::new(0.75, 0.0, 0.25),
                top_right: Vector3::new(0.75, 0.0, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.0, 0.25),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.0, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.0, 0.25),
                top_right: Vector3::new(0.75, 0.0, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.75, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.0, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.0, 0.75),
                top_right: Vector3::new(0.75, 0.0, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.0, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.0, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.0, 0.25),
                top_right: Vector3::new(0.25, 0.0, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.0, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
        // Left
        [
            Face {
                top_left: Vector3::new(0.0, 0.75, 0.25),
                top_right: Vector3::new(0.0, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.0, 0.25, 0.25),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.0, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.0, 0.25, 0.25),
                top_right: Vector3::new(0.0, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.0, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.0, 0.75, 0.75),
                top_right: Vector3::new(0.0, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.0, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.0, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.0, 0.75, 0.25),
                top_right: Vector3::new(0.0, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.0, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
        // Right
        [
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(1.0, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(1.0, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(1.0, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(1.0, 0.25, 0.75),
                top_right: Vector3::new(1.0, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(1.0, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(1.0, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(1.0, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(1.0, 0.75, 0.75),
                top_right: Vector3::new(1.0, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
        // Front
        [
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.0),
                top_right: Vector3::new(0.75, 0.75, 0.0),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.0),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.0),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.0),
                top_right: Vector3::new(0.25, 0.75, 0.0),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.0),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.0),
                top_right: Vector3::new(0.25, 0.75, 0.0),
                bottom_left: Vector3::new(0.75, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.0),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.0),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.0),
                top_right: Vector3::new(0.25, 0.25, 0.0),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.75, 0.0),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
        // Back
        [
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 1.0),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 1.0),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.25, 1.0),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 1.0),
                top_right: Vector3::new(0.75, 0.75, 1.0),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 1.0),
                texture,
                direction: ViewableDirectionBitMap::Bottom,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(0.75, 0.25, 1.0),
                bottom_left: Vector3::new(0.25, 0.25, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Top,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 1.0),
                texture,
                direction: ViewableDirectionBitMap::Left,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
            Face {
                top_left: Vector3::new(0.75, 0.75, 1.0),
                top_right: Vector3::new(0.75, 0.25, 1.0),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture,
                direction: ViewableDirectionBitMap::Right,
                edge: false,
                normal: ViewableDirectionBitMap::Top.to_normal(),
                wind_strengths: None
            },
        ],
    ]
}
