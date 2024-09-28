use crate::game::generation::noise::SimplexNoise;
use nalgebra::Vector3;
use rc_shared::biome::EnvironmentMap;
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;
use rc_shared::relative_chunk_flat_map::RelativeChunkFlatMap;
use crate::game::generation::structures::{StructureBoundingBox, StructureGenerator};
use crate::game::generation::structures::tree::TreeStructureGenerator;

/// Adds structures to chunk
/// `world_pos` is the position of `world`, while `generation_pos` is a surrounding chunk that is having its structures generated also to generate overlap
/// in the chunks structures.
pub fn add_structures(
    seed: u32,
    chunk_pos: Vector3<i32>,
    world: &mut RawChunkData,
    heightmap: &RelativeChunkFlatMap<i32>,
    environment: &EnvironmentMap
) {

    let mut trees = vec![];

    let chunk_bounding_box = StructureBoundingBox::new(
        chunk_pos * CHUNK_SIZE as i32,
        Vector3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32)
    );

    let tree_noise = SimplexNoise::new(seed + 10).with_scale(1.0);

    let world_pos = chunk_pos * CHUNK_SIZE as i32;

    for x in (world_pos.x - CHUNK_SIZE as i32)..(world_pos.x + (2*CHUNK_SIZE as i32)) {
        for z in (world_pos.z - CHUNK_SIZE as i32)..(world_pos.z + (2*CHUNK_SIZE as i32)) {
            let ground_level = *heightmap.get([x, z]).unwrap();

            if ground_level > 17 {
                continue;
            }

            let environment = *environment.get([x, z]).unwrap();

            let affects_chunk =
                chunk_bounding_box.collides(&TreeStructureGenerator::bounding_box().shifted(Vector3::new(x, ground_level + 1, z)));

            let intersects_tree = does_tree_intersect(&trees, Vector3::new(x, ground_level + 1, z));

            if environment.vegetation > 0.3
                && !intersects_tree && affects_chunk
                && tree_noise.sample_2d(x as f64, z as f64) > 0.9
            {
                trees.push(Vector3::new(x, ground_level + 1, z));
                TreeStructureGenerator::spawn(seed, chunk_pos, Vector3::new(x, ground_level + 1, z), world);
            }
        }
    }
}

fn does_tree_intersect(trees: &Vec<Vector3<i32>>, pos: Vector3<i32>) -> bool {
    for tree in trees {
        if TreeStructureGenerator::bounding_box().shifted(pos).collides(
            &TreeStructureGenerator::bounding_box().shifted(*tree)
        ) {
            return true
        }
    }
    false
}