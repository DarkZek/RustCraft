use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::*;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::helpers::global_to_local_position;

pub struct RaycastResult {
    pub block: Vector3<i32>,
    pub normal: Vector3<i32>,
    pub distance: f32,
}

/// Attempts a raycast of the terrain
pub fn do_raycast(
    starting_position: Vector3<f32>,
    mut direction: Vector3<f32>,
    max_distance: f32,
    chunks: &ChunkSystem,
    blocks: &BlockStates,
) -> Option<RaycastResult> {
    direction = direction.normalize();

    let mut last_chunk: Option<&ChunkData> = None;

    let mut block = Vector3::new(
        starting_position.x.floor() as i32,
        starting_position.y.floor() as i32,
        starting_position.z.floor() as i32,
    );

    let step = Vector3::new(
        signum(direction.x),
        signum(direction.y),
        signum(direction.z),
    );

    let delta = Vector3::new(
        (1.0 / direction.x).abs(),
        (1.0 / direction.y).abs(),
        (1.0 / direction.z).abs(),
    );

    let dist = Vec3::new(
        if step.x > 0 {
            block.x as f32 - starting_position.x + 1.0
        } else {
            starting_position.x - block.x as f32
        },
        if step.y > 0 {
            block.y as f32 - starting_position.y + 1.0
        } else {
            starting_position.y - block.y as f32
        },
        if step.z > 0 {
            block.z as f32 - starting_position.z + 1.0
        } else {
            starting_position.z - block.z as f32
        },
    );

    // The nearest voxel boundary.
    let mut t_max = Vec3::new(delta.x * dist.x, delta.y * dist.y, delta.z * dist.z);

    let mut distance = 0.0;

    // Used to create normal
    let mut last_position = block;

    while distance < max_distance {
        let (chunk_pos, local_pos) = global_to_local_position(block);

        // Use last chunk if its the same chunk, otherwise fetch new chunk
        last_chunk = if let Some(c) = &last_chunk.filter(|v| v.position == chunk_pos) {
            Some(*c)
        } else {
            chunks.chunks.get(&chunk_pos)
        };

        // Check if block is solid
        if let Some(chunk_data) = last_chunk {
            let block_id = chunk_data.world[local_pos.x][local_pos.y][local_pos.z];
            if block_id != 0 {
                let collided_block = blocks.get_block(block_id as usize);

                // Don't bother real aabb check if we know its a full block
                if collided_block.full {
                    // Solid block
                    return Some(RaycastResult {
                        block,
                        normal: last_position - block,
                        distance,
                    });
                }

                // Loop through colliders on the block and see if the blocks collider is hit
                for collider in &collided_block.bounding_boxes {
                    if collider
                        .offset(block.cast::<f32>())
                        .ray_collides(starting_position, direction)
                        .0
                    {
                        return Some(RaycastResult {
                            block,
                            normal: last_position - block,
                            distance,
                        });
                    }
                }
            }
        }

        last_position = block;

        // Move one block
        if t_max.x < t_max.y {
            if t_max.x < t_max.z {
                block.x += step.x;
                t_max.x += delta.x;
            } else {
                block.z += step.z;
                t_max.z += delta.z;
            }
        } else {
            if t_max.y < t_max.z {
                block.y += step.y;
                t_max.y += delta.y;
            } else {
                block.z += step.z;
                t_max.z += delta.z;
            }
        }

        distance = (starting_position - block.cast::<f32>()).magnitude();
    }

    None
}

fn signum(n: f32) -> i32 {
    if n > 0.0 {
        1
    } else if n == 0.0 {
        0
    } else {
        -1
    }
}
