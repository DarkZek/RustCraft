
use bevy::ecs::prelude::Component;
use nalgebra::Vector3;
use rc_networking::constants::CHUNK_SIZE;

#[derive(Debug, Component)]
pub struct ChunkData {
    pub position: Vector3<i32>,

    pub world: RawChunkData,
}

pub type RawChunkData = [[[u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

impl ChunkData {
    pub fn new(position: Vector3<i32>, world: RawChunkData) -> ChunkData {
        ChunkData { position, world }
    }
    pub fn blank(position: Vector3<i32>) -> ChunkData {
        ChunkData {
            position,
            world: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn generate(position: Vector3<i32>) -> ChunkData {
        let mut world = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if (x + y + z) % 3 == 0 {
                        world[x][y][z] = 1;
                    } else {
                        world[x][y][z] = 2;
                    }
                }
            }
        }

        ChunkData { position, world }
    }

    // pub fn send(&self, server: &mut Server<Protocol, Channels>, key: &UserKey) {
    //     let sections = PartialChunkUpdate::generate(&self.world, [self.position.x, self.position.y, self.position.z]);
    //
    //     for packet in sections {
    //         server.send_message(key, Channels::ChunkUpdates, &packet);
    //     }
    // }
}
