#![feature(duration_millis_float)]

use std::fs;
use std::time::Instant;
use nalgebra::Vector3;
use rc_client::systems::chunk::data::ChunkData;
use rc_client::systems::chunk::nearby_cache::NearbyChunkCache;
use rc_shared::block::BlockStates;
use rc_shared::block::types::Block;
use rc_shared::CHUNK_SIZE;

fn main() {

    let mut chunk_data = [[[0; 16]; 16]; 16];

    // Light source
    chunk_data[2][4][4] = 2;

    let chunk_pos = Vector3::new(0, 0, 0);
    let chunk = ChunkData::new_handleless(chunk_data, chunk_pos);

    let cache = NearbyChunkCache::empty(chunk_pos);
    let mut states = BlockStates::new();

    // Add sample blocks
    states.states.push(Block {
        identifier: "mcv3::Air".to_string(),
        translucent: true,
        full: false,
        draw_betweens: false,
        faces: vec![],
        collision_boxes: vec![],
        bounding_boxes: vec![],
        emission: [0; 4],
    });
    states.states.push(Block {
        identifier: "mcv3::Test".to_string(),
        translucent: false,
        full: true,
        draw_betweens: false,
        faces: vec![],
        collision_boxes: vec![],
        bounding_boxes: vec![],
        emission: [0; 4],
    });
    states.states.push(Block {
        identifier: "mcv3::Light".to_string(),
        translucent: false,
        full: true,
        draw_betweens: false,
        faces: vec![],
        collision_boxes: vec![],
        bounding_boxes: vec![],
        emission: [255, 255, 255, 255],
    });


    let start = Instant::now();
    for i in 0..1 {
        let mut out = [[[[0; 4]; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let lighting_update = chunk.build_lighting_blur(&states, &cache, &mut out);

        if i == 0 {
            fs::write("out.txt", format!("{:?}", out));
        }
    }
    let elapsed = start.elapsed();
    println!("Time {}ns {}ms", elapsed.as_nanos(), elapsed.as_millis_f64());
}