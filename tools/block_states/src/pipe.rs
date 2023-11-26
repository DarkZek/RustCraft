use crate::loading::{
    BlockStatesFile, DeserialisedAabb, DeserialisedBlock, DeserialisedFace,
    DeserialisedLootTableEntry,
};
use nalgebra::Vector3;
use std::fs;

pub const BLOCK_SIDES: [Vector3<i32>; 6] = [
    Vector3::new(0, 1, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(-1, 0, 0),
    Vector3::new(1, 0, 0),
    Vector3::new(0, 0, -1),
    Vector3::new(0, 0, 1),
];

pub fn pipe() -> Vec<DeserialisedBlock> {
    let mut states = Vec::new();

    let block_placeholder = DeserialisedBlock {
        identifier: "mcv3::Pipe".to_string(),
        translucent: true,
        full: false,
        draw_betweens: false,
        faces: vec![
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.25),
                top_right: Vector3::new(0.25, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
        ],
        colliders: vec![DeserialisedAabb {
            bottom_left: Vector3::new(0.25, 0.25, 0.0),
            size: Vector3::new(0.5, 0.5, 1.0),
            collidable: true,
        }],
        emission: [0, 0, 0, 0],
        loot_table: vec![DeserialisedLootTableEntry {
            chance: 2.0,
            item: "mcv3::PipeItem".to_string(),
        }],
    };

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

        let mut working_copy = block_placeholder.clone();

        for x in 0..6 {
            if sides_active[x] == false {
                continue;
            }

            working_copy
                .faces
                .append(&mut data().get_mut(x).unwrap().to_vec());
        }

        states.push(working_copy);
    }

    states
}

fn data<'a>() -> Vec<[DeserialisedFace; 8]> {
    vec![
        // Top
        [
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 1.0, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.25),
                top_right: Vector3::new(0.25, 1.0, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 1.0, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 1.0, 0.75),
                top_right: Vector3::new(0.75, 1.0, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 1.0, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 1.0, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.25, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 1.0, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 1.0, 0.75),
                top_right: Vector3::new(0.25, 1.0, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
        // Bottom
        [
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.0, 0.25),
                top_right: Vector3::new(0.75, 0.0, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.0, 0.25),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.0, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.0, 0.25),
                top_right: Vector3::new(0.75, 0.0, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.75, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.0, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.0, 0.75),
                top_right: Vector3::new(0.75, 0.0, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.0, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.0, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.0, 0.25),
                top_right: Vector3::new(0.25, 0.0, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.0, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
        // Left
        [
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.75, 0.25),
                top_right: Vector3::new(0.0, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.25, 0.25),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.0, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.25, 0.25),
                top_right: Vector3::new(0.0, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.0, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.75, 0.75),
                top_right: Vector3::new(0.0, 0.75, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.0, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.0, 0.75, 0.25),
                top_right: Vector3::new(0.0, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.0, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
        // Right
        [
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(1.0, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(1.0, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(1.0, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(1.0, 0.25, 0.75),
                top_right: Vector3::new(1.0, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(1.0, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(1.0, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(1.0, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(1.0, 0.75, 0.75),
                top_right: Vector3::new(1.0, 0.25, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
        // Front
        [
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.0),
                top_right: Vector3::new(0.75, 0.75, 0.0),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.0),
                top_right: Vector3::new(0.25, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.0),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.0),
                top_right: Vector3::new(0.25, 0.75, 0.0),
                bottom_left: Vector3::new(0.25, 0.25, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.25),
                top_right: Vector3::new(0.75, 0.75, 0.25),
                bottom_left: Vector3::new(0.75, 0.25, 0.0),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.0),
                top_right: Vector3::new(0.25, 0.75, 0.0),
                bottom_left: Vector3::new(0.75, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.0),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.25, 0.25, 0.0),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.0),
                top_right: Vector3::new(0.25, 0.25, 0.0),
                bottom_left: Vector3::new(0.25, 0.75, 0.25),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.25),
                top_right: Vector3::new(0.75, 0.25, 0.25),
                bottom_left: Vector3::new(0.75, 0.75, 0.0),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
        // Back
        [
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.75, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 1.0),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.25, 1.0),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.25, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.25, 0.25, 1.0),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 1.0),
                top_right: Vector3::new(0.75, 0.75, 1.0),
                bottom_left: Vector3::new(0.75, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.75, 0.75),
                bottom_left: Vector3::new(0.75, 0.75, 1.0),
                texture: "game/pipe_end".to_string(),
                direction: 2,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.25, 0.75),
                top_right: Vector3::new(0.75, 0.25, 1.0),
                bottom_left: Vector3::new(0.25, 0.25, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 1,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.25, 0.75, 0.75),
                top_right: Vector3::new(0.25, 0.25, 0.75),
                bottom_left: Vector3::new(0.25, 0.75, 1.0),
                texture: "game/pipe_end".to_string(),
                direction: 4,
                edge: false,
            },
            DeserialisedFace {
                top_left: Vector3::new(0.75, 0.75, 1.0),
                top_right: Vector3::new(0.75, 0.25, 1.0),
                bottom_left: Vector3::new(0.75, 0.75, 0.75),
                texture: "game/pipe_end".to_string(),
                direction: 8,
                edge: false,
            },
        ],
    ]
}
