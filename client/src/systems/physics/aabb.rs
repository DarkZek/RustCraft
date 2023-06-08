use crate::game::blocks::states::BlockStates;
use crate::helpers::{get_chunk_coords, global_to_local_position};
use crate::systems::chunk::data::ChunkData;
use crate::systems::chunk::ChunkSystem;
use bevy::prelude::{Color, ResMut, Vec3};
use bevy_prototype_debug_lines::DebugLines;
use nalgebra::{clamp, Vector3};
use serde::{Deserialize, Serialize};

#[inline(always)]
fn max(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}
#[inline(always)]
fn min(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Aabb {
    pub bottom_left: Vector3<f32>,
    pub size: Vector3<f32>,
}

impl Aabb {
    pub fn new(bottom_left: Vector3<f32>, size: Vector3<f32>) -> Aabb {
        Aabb { bottom_left, size }
    }

    pub fn offset(&self, offset: Vector3<f32>) -> Self {
        let mut new = self.clone();
        new.bottom_left += offset;
        new
    }

    pub fn draw_lines(boxes: &Vec<Aabb>, position: Vector3<f32>, lines: &mut ResMut<DebugLines>) {
        for val in boxes {
            val.offset(position).draw(lines, 0.0, Color::WHITE);
        }
    }

    /// Draw an outline of a Aabb collider using DebugLines
    pub fn draw(&self, lines: &mut ResMut<DebugLines>, duration: f32, color: Color) {
        let base = self.bottom_left;

        // Bottom ring
        lines.line_colored(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x + self.size.x, base.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x, base.y, base.z + self.size.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(base.x + self.size.x, base.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(base.x, base.y, base.z + self.size.z),
            duration,
            color,
        );

        // Top ring
        lines.line_colored(
            Vec3::new(base.x, base.y + self.size.y, base.z),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x, base.y + self.size.y, base.z),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
            color,
        );

        // Vertical ring
        lines.line_colored(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x, base.y + self.size.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x, base.y, base.z + self.size.z),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x + self.size.x, base.y, base.z),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
            color,
        );
        lines.line_colored(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            duration,
            color,
        );
    }

    /// Detect if a ray collides with this Aabb
    /// Returns a boolean which is true if a hit was detected, and the distance
    pub fn ray_collides(&self, origin: Vector3<f32>, direction: Vector3<f32>) -> (bool, f32) {
        let inv_direction = Vector3::new(
            if direction.x == 0.0 {
                0.0
            } else {
                1.0 / direction.x
            },
            if direction.y == 0.0 {
                0.0
            } else {
                1.0 / direction.y
            },
            if direction.z == 0.0 {
                0.0
            } else {
                1.0 / direction.z
            },
        );

        let aabb_max = self.bottom_left + self.size;
        let mut tmin = 0.0;
        let mut tmax = f32::INFINITY;

        let tx1 = (self.bottom_left.x - origin.x) * inv_direction.x;
        let tx2 = (aabb_max.x - origin.x) * inv_direction.x;

        tmin = min(max(tx1, tmin), max(tx2, tmin));
        tmax = max(min(tx1, tmax), min(tx2, tmax));

        let ty1 = (self.bottom_left.y - origin.y) * inv_direction.y;
        let ty2 = (aabb_max.y - origin.y) * inv_direction.y;

        tmin = min(max(ty1, tmin), max(ty2, tmin));
        tmax = max(min(ty1, tmax), min(ty2, tmax));

        let tz1 = (self.bottom_left.z - origin.z) * inv_direction.z;
        let tz2 = (aabb_max.z - origin.z) * inv_direction.z;

        tmin = min(max(tz1, tmin), max(tz2, tmin));
        tmax = max(min(tz1, tmax), min(tz2, tmax));

        if tmax < 0.0 {
            (false, tmax)
        } else if tmin > tmax {
            // if tmin > tmax, ray doesn't intersect AABB
            (false, tmax)
        } else {
            (true, tmin)
        }
    }

    /// Check if this Aabb intersects with `other`
    pub fn aabb_collides(&self, other: &Self) -> bool {
        let x_check_1 = other.bottom_left.x > self.bottom_left.x
            && other.bottom_left.x < self.bottom_left.x + self.size.x;
        let x_check_2 = other.bottom_left.x + other.size.x > self.bottom_left.x
            && other.bottom_left.x + other.size.x < self.bottom_left.x + self.size.x;
        let y_check_1 = other.bottom_left.y > self.bottom_left.y
            && other.bottom_left.y < self.bottom_left.y + self.size.y;
        let y_check_2 = other.bottom_left.y + other.size.y > self.bottom_left.y
            && other.bottom_left.y + other.size.y < self.bottom_left.y + self.size.y;
        let z_check_1 = other.bottom_left.z > self.bottom_left.z
            && other.bottom_left.z < self.bottom_left.z + self.size.z;
        let z_check_2 = other.bottom_left.z + other.size.z > self.bottom_left.z
            && other.bottom_left.z + other.size.z < self.bottom_left.z + self.size.z;

        x_check_1 || x_check_2 || y_check_1 || y_check_2 || z_check_1 || z_check_2
    }

    pub fn get_voxel_collision_coords(
        &self,
        chunks: &ChunkSystem,
        blocks: &BlockStates,
    ) -> Vec<Vector3<i32>> {
        let mut matches = Vec::new();

        let mut previous_chunk: Option<&ChunkData> = None;
        // Loop over all potential blocks
        for x in (f32::floor(self.bottom_left.x) as i32)
            ..(f32::ceil(self.bottom_left.x + self.size.x) as i32)
        {
            for y in (f32::floor(self.bottom_left.y) as i32)
                ..(f32::ceil(self.bottom_left.y + self.size.y) as i32)
            {
                for z in (f32::floor(self.bottom_left.z) as i32)
                    ..(f32::ceil(self.bottom_left.z + self.size.z) as i32)
                {
                    let block: Vector3<i32> = Vector3::new(x, y, z);
                    let (chunk_pos, block_pos) = global_to_local_position(block);

                    // Chunk caching to speed things up
                    if previous_chunk.is_none() || previous_chunk.unwrap().position != chunk_pos {
                        previous_chunk = chunks.chunks.get(&chunk_pos);
                    }

                    if let Some(chunk_data) = previous_chunk {
                        // Fetch block id
                        let block_id = chunk_data.world[block_pos.x][block_pos.y][block_pos.z];

                        // Fetch block information
                        let block_data = blocks.get_block(block_id as usize);

                        for collider in &block_data.bounding_boxes {
                            let collider = collider.offset(Vector3::new(
                                block.x as f32,
                                block.y as f32,
                                block.z as f32,
                            ));
                            if collider.aabb_collides(self) {
                                // Collision!
                                matches.push(block);

                                // No need to check other colliders for this block
                                break;
                            }
                        }
                    }
                }
            }
        }

        matches
    }

    pub fn get_voxel_collision_colliders(
        &self,
        chunks: &ChunkSystem,
        blocks: &BlockStates,
    ) -> Vec<Aabb> {
        let mut matches = Vec::new();

        let mut previous_chunk: Option<&ChunkData> = None;
        // Loop over all potential blocks
        for x in (f32::floor(self.bottom_left.x) as i32)
            ..(f32::ceil(self.bottom_left.x + self.size.x) as i32)
        {
            for y in (f32::floor(self.bottom_left.y) as i32)
                ..(f32::ceil(self.bottom_left.y + self.size.y) as i32)
            {
                for z in (f32::floor(self.bottom_left.z) as i32)
                    ..(f32::ceil(self.bottom_left.z + self.size.z) as i32)
                {
                    let block: Vector3<i32> = Vector3::new(x, y, z);
                    let (chunk_pos, block_pos) = global_to_local_position(block);

                    // Chunk caching to speed things up
                    if previous_chunk.is_none() || previous_chunk.unwrap().position != chunk_pos {
                        previous_chunk = chunks.chunks.get(&chunk_pos);
                    }

                    if let Some(chunk_data) = previous_chunk {
                        // Fetch block id
                        let block_id = chunk_data.world[block_pos.x][block_pos.y][block_pos.z];

                        // Fetch block information
                        let block_data = blocks.get_block(block_id as usize);

                        for collider in &block_data.bounding_boxes {
                            let collider = collider.offset(Vector3::new(
                                block.x as f32,
                                block.y as f32,
                                block.z as f32,
                            ));
                            if collider.aabb_collides(self) {
                                // Collision!
                                matches.push(collider);
                            }
                        }
                    }
                }
            }
        }

        matches
    }

    /// Only one axis at a time
    pub fn try_move(&self, movement: Vector3<f32>, other: &Aabb) -> Vector3<f32> {
        println!("try_move({:?}, {:?}, {:?})", self, movement, other);
        let corners = [
            self.bottom_left,
            self.bottom_left + Vector3::new(self.size.x, 0.0, 0.0),
            self.bottom_left + Vector3::new(0.0, 0.0, self.size.z),
            self.bottom_left + Vector3::new(self.size.x, 0.0, self.size.z),
            self.bottom_left + Vector3::new(0.0, self.size.y, 0.0),
            self.bottom_left + Vector3::new(self.size.x, self.size.y, 0.0),
            self.bottom_left + Vector3::new(0.0, self.size.y, self.size.z),
            self.bottom_left + Vector3::new(self.size.x, self.size.y, self.size.z),
        ];

        let mut new_movement = movement.clone();

        for point in corners {
            let (hit, dist) = other.ray_collides(point, movement.normalize());
            if hit {
                println!(
                    "Dist {:?} Dir {:?} Player Point {:?} Block Collider: {:?}\n\tray_collides({:?}, {:?})",
                    dist, new_movement, point, other, point + Vector3::new(0.0, 0.01, 0.0), movement.normalize()
                );
                if movement.x != 0.0 {
                    new_movement.x = clamp(movement.x, -dist, dist);
                }
                if movement.y != 0.0 {
                    new_movement.y = clamp(movement.y, -dist, dist);
                }
                if movement.z != 0.0 {
                    new_movement.z = clamp(movement.z, -dist, dist);
                }
            }
        }

        new_movement
    }
}
