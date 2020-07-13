use crate::game::physics::collider::BoxCollider;
use crate::services::chunk_service::chunk::{ChunkData};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::Point3;

impl ChunkData {
    /// Calculates a chunks colliders from its geometry. It is a lot like greedy meshing, except with colliders.
    pub fn calculate_colliders(&mut self) {

        let mut colliders = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.world[x][y][z] == 0 {
                        continue;
                    }

                    colliders.push(BoxCollider {
                        p1: Point3::new(x as f32, y as f32, z as f32),
                        p2: Point3::new(x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0),
                        center: Point3::new(x as f32, y as f32, z as f32),
                    })
                }
            }
        }

        self.collision_map = colliders;
    }
}
