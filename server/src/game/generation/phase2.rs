use std::ops::Range;

use crate::game::generation::noise::SimplexNoise;
use nalgebra::{Vector2, Vector3};
use rc_shared::biome::{EnvironmentMap};
use rc_shared::chunk::RawChunkData;
use rc_shared::CHUNK_SIZE;
use ::noise::{RidgedMulti, Perlin};
use ::noise::{MultiFractal, NoiseFn, Seedable, Turbulence};
use rc_shared::relative_chunk_flat_map::RelativeChunkFlatMap;

pub fn generate_greybox_chunk(
    seed: u32,
    pos: Vector3<i32>,
    environment: &EnvironmentMap,
) -> (RawChunkData, RelativeChunkFlatMap<i32>) {
    let ground_noise = SimplexNoise::new(0).with_scale(30.0);
    let ground_noise_2 = SimplexNoise::new(100).with_scale(50.0);
    let ground_noise_3 = SimplexNoise::new(200).with_scale(100.0);

    let world_pos = pos * CHUNK_SIZE as i32;

    let mut heightmap = RelativeChunkFlatMap::new_empty(Vector2::new(world_pos.x, world_pos.z), CHUNK_SIZE);

    for x in (world_pos.x - CHUNK_SIZE as i32)..(world_pos.x + (CHUNK_SIZE as i32 * 2)) {
        for z in (world_pos.z - CHUNK_SIZE as i32)..(world_pos.z + (CHUNK_SIZE as i32 * 2)) {
            let base_height = 35;

            let environment_entry = environment.get([x, z]).unwrap();

            // Hilly
            let height_multiplier = 8.0 + clamp_map(0.5..1.0, 0.0..32.0, environment_entry.terrain);

            let ground_level = (ground_noise_3.sample_2d(x, z)
                * height_multiplier
                + environment_entry.terrain * 10.0
                + ground_noise_2.sample_2d(x, z) * 5.0
                + ground_noise.sample_2d(x, z) * 2.0)
                .floor() as i32;

            heightmap.set([x, z], ground_level);
        }
    }


    let scale = 0.05;

    let primary_jade = RidgedMulti::<Perlin>::new(seed)
        .set_lacunarity(2.20703125)
        .set_octaves(1);

    let perturbed_base_secondary_jade = Turbulence::<_, Perlin>::new(primary_jade)
        .set_seed(seed)
        .set_frequency(2.0)
        .set_power(1.0 / 16.0)
        .set_roughness(4);

    let mut world = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let absolute = Vector3::new(
                    (pos.x * 16) + x as i32,
                    (pos.y * 16) + y as i32,
                    (pos.z * 16) + z as i32,
                );
                let v = perturbed_base_secondary_jade.get([absolute.x as f64 * scale, absolute.z as f64 * scale, absolute.y as f64 * scale * 2.0]);

                if absolute.y < *heightmap.get([absolute.x, absolute.z]).unwrap() && v < 0.7 {
                    world[x][y][z] = 6;
                }
            }
        }
    }

    

    (world, heightmap)
}

fn clamp_map<T: Into<f64>>(input: Range<f64>, output: Range<f64>, v: T) -> f64 {
    let mut normalized = (v.into() - input.start) / (input.end - input.start);

    normalized = normalized.max(1.0).min(0.0);

    (normalized * (output.end - output.start)) + output.start
}