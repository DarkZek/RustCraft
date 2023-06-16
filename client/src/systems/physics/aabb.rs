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

    /// Offset an Aabb by a given offset
    pub fn offset(&self, offset: Vector3<f32>) -> Self {
        let mut new = self.clone();
        new.bottom_left += offset;
        new
    }

    /// Draws a `Aabb` outline with DebugLines
    pub fn draw_lines(boxes: &Vec<Aabb>, position: Vector3<f32>, lines: &mut ResMut<DebugLines>) {
        for val in boxes {
            val.offset(position).draw(lines, 0.0, Color::WHITE);
        }
    }

    /// Draw an outline of a Aabb collider using DebugLines
    pub fn draw(&self, lines: &mut DebugLines, duration: f32, color: Color) {
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
        let x_check_1 = other.bottom_left.x >= self.bottom_left.x + self.size.x;
        let x_check_2 = other.bottom_left.x + other.size.x <= self.bottom_left.x;

        let y_check_1 = other.bottom_left.y >= self.bottom_left.y + self.size.y;
        let y_check_2 = other.bottom_left.y + other.size.y <= self.bottom_left.y;

        let z_check_1 = other.bottom_left.z >= self.bottom_left.z + self.size.z;
        let z_check_2 = other.bottom_left.z + other.size.z <= self.bottom_left.z;

        !(x_check_1 || x_check_2 || y_check_1 || y_check_2 || z_check_1 || z_check_2)
    }

    /// Get the colliders of all blocks that could come in contact with an `Aabb`
    pub fn get_surrounding_voxel_collision_colliders(
        &self,
        chunks: &ChunkSystem,
        blocks: &BlockStates,
    ) -> Vec<Aabb> {
        let mut matches = Vec::new();

        let mut previous_chunk: Option<&ChunkData> = None;
        // Loop over all potential blocks
        for x in (f32::floor(self.bottom_left.x) as i32 - 1)
            ..(f32::ceil(self.bottom_left.x + self.size.x) as i32 + 1)
        {
            for y in (f32::floor(self.bottom_left.y) as i32 - 1)
                ..(f32::ceil(self.bottom_left.y + self.size.y) as i32 + 1)
            {
                for z in (f32::floor(self.bottom_left.z) as i32 - 1)
                    ..(f32::ceil(self.bottom_left.z + self.size.z) as i32 + 1)
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

                        for collider in &block_data.collision_boxes {
                            let collider = collider.offset(Vector3::new(
                                block.x as f32,
                                block.y as f32,
                                block.z as f32,
                            ));
                            matches.push(collider);
                        }
                    }
                }
            }
        }

        matches
    }

    /// Returns the maximum movement allowed before collision with another `Aabb`
    pub fn try_move(&self, mut movement: Vector3<f32>, other: &Aabb) -> Vector3<f32> {
        if movement.x > 0.0 {
            let mx = self.bottom_left.x + self.size.x + movement.x;
            movement.x -= mx - other.bottom_left.x + 0.0001;
        } else if movement.x < 0.0 {
            let mx = self.bottom_left.x + movement.x;
            movement.x -= mx - (other.bottom_left.x + other.size.x) - 0.0001;
        }
        if movement.y > 0.0 {
            let my = self.bottom_left.y + self.size.y + movement.y;
            movement.y -= my - other.bottom_left.y + 0.0001;
        } else if movement.y < 0.0 {
            let my = self.bottom_left.y + movement.y;
            movement.y -= my - (other.bottom_left.y + other.size.y) - 0.0001;
        }
        if movement.z > 0.0 {
            let mz = self.bottom_left.z + self.size.z + movement.z;
            movement.z -= mz - other.bottom_left.z + 0.0001;
        } else if movement.z < 0.0 {
            let mz = self.bottom_left.z + movement.z;
            movement.z -= mz - (other.bottom_left.z + other.size.z) - 0.0001;
        }

        movement
    }

    /// Attempt to translate an Aabb by a delta, moving the maximum distance allowed before a collision
    /// Supports movement on only one axis at a time
    pub fn try_translate(
        &self,
        mut proposed_delta: Vector3<f32>,
        colliders: &Vec<Aabb>,
    ) -> Vector3<f32> {
        let mut proposed_aabb = self.offset(proposed_delta);

        for block in colliders {
            if !block.aabb_collides(&proposed_aabb) {
                continue;
            }

            // Previous delta change could have made the move redundant
            if block.aabb_collides(&proposed_aabb) {
                proposed_delta = self.try_move(proposed_delta, &block);
            }
            proposed_aabb = self.offset(proposed_delta);
        }

        proposed_delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_inspector_egui::egui::Shape::Vec;

    #[test]
    fn aabb_vs_aabb_move_test_x() {
        let player = Aabb::new(Vector3::new(1.2, 0.0, 0.5), Vector3::new(1.0, 2.0, 1.0));
        let block = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let new_pos = player.try_move(Vector3::new(-0.5, 0.0, 0.0), &block);
        assert!(
            new_pos.x > -0.2 && new_pos.x < -0.199,
            "value {}",
            new_pos.x
        );
        assert_eq!(new_pos.y, 0.0);
        assert_eq!(new_pos.z, 0.0);
    }
    #[test]
    fn aabb_vs_aabb_move_test_y() {
        let player = Aabb::new(Vector3::new(0.5, 1.1, 0.5), Vector3::new(1.0, 2.0, 1.0));
        let block = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let new_pos = player.try_move(Vector3::new(0.0, -0.2, 0.0), &block);
        assert_eq!(new_pos.x, 0.0);
        assert!(
            new_pos.y > -0.1 && new_pos.y < -0.0999,
            "value {}",
            new_pos.y
        );
        assert_eq!(new_pos.z, 0.0);
    }
    #[test]
    fn aabb_vs_aabb_move_test_z() {
        let player = Aabb::new(Vector3::new(0.5, 0.0, 1.1), Vector3::new(1.0, 2.0, 1.0));
        let block = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let new_pos = player.try_move(Vector3::new(0.0, 0.0, -0.2), &block);
        assert_eq!(new_pos.x, 0.0);
        assert_eq!(new_pos.y, 0.0);
        assert!(
            new_pos.z > -0.1 && new_pos.z < -0.0999,
            "value {}",
            new_pos.z
        );
    }
    #[test]
    fn aabb_vs_aabb_move_test_neg_x() {
        let player = Aabb::new(Vector3::new(-1.2, 0.0, 0.5), Vector3::new(1.0, 2.0, 1.0));
        let block = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let new_pos = player.try_move(Vector3::new(0.5, 0.0, 0.0), &block);
        assert!(new_pos.x > 0.199 && new_pos.x < 0.2, "value {}", new_pos.x);
        assert_eq!(new_pos.y, 0.0);
        assert_eq!(new_pos.z, 0.0);
    }

    #[test]
    fn aabb_vs_aabb_test_detection_1() {
        let player = Aabb::new(
            Vector3::new(0.31867227, 36.0001, 0.0),
            Vector3::new(0.7, 1.85, 0.7),
        );
        let block = Aabb::new(Vector3::new(1.0, 35.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        assert!(!player.aabb_collides(&block));
        assert!(!block.aabb_collides(&player));
    }
    #[test]
    fn aabb_vs_aabb_test_detection_2() {
        let player = Aabb::new(
            Vector3::new(-17.618488, 38.0001, 11.824133),
            Vector3::new(0.7, 1.85, 0.7),
        );
        let block = Aabb::new(Vector3::new(-18.0, 37.0, 12.0), Vector3::new(1.0, 1.0, 1.0));
        assert!(!player.aabb_collides(&block));
        assert!(!block.aabb_collides(&player));
    }
    #[test]
    fn ray_vs_aabb_test_detection_1() {
        let block = Aabb::new(Vector3::new(0.0, 34.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let dir = Vector3::new(0.23533043, -0.9655777, -0.110811174);
        let pos = Vector3::new(-0.6499001, 35.700104, 0.30660504);
        assert!(!block.ray_collides(pos, dir).0);
    }
    #[test]
    fn ray_vs_aabb_test_detection_2() {
        let block = Aabb::new(Vector3::new(0.0, 32.0, -2.0), Vector3::new(1.0, 1.0, 1.0));
        let dir = Vector3::new(0.026328959, -0.82820773, -0.5598024);
        let pos = Vector3::new(-0.1359614, 35.7001, 0.23648061);
        assert!(block.ray_collides(pos, dir).0);
    }
}
