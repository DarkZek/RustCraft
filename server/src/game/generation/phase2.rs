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
    let ground_noise_4 = SimplexNoise::new(150).with_scale(5.0);

    let world_pos = pos * CHUNK_SIZE as i32;

    let mut heightmap = RelativeChunkFlatMap::new_empty(Vector2::new(world_pos.x, world_pos.z), CHUNK_SIZE);

    for x in (world_pos.x - CHUNK_SIZE as i32)..(world_pos.x + (CHUNK_SIZE as i32 * 2)) {
        for z in (world_pos.z - CHUNK_SIZE as i32)..(world_pos.z + (CHUNK_SIZE as i32 * 2)) {
            let base_height = 35;

            let environment_entry = environment.get([x, z]).unwrap();

            // Hilly
            let height_multiplier = 8.0 +
                (
                    clamp_map(0.5..1.0, 0.0..8.0, environment_entry.terrain).powf(3.0)
                    + clamp_map(0.3..0.7, 0.0..4.0, ground_noise_4.sample_2d(x, z))
                );

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

    let caves = Turbulence::<_, Perlin>::new(primary_jade)
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

                let v = caves.get([absolute.x as f64 * scale, absolute.z as f64 * scale, absolute.y as f64 * scale * 2.0]);

                let threshold = clamp_map(
                    10.0..25.0,
                    0.7..1.0,
                    absolute.y as f32
                );

                // Set stone of world
                if absolute.y < *heightmap.get([absolute.x, absolute.z]).unwrap() && v < threshold {
                    world[x][y][z] = 6;
                }
            }
        }
    }

    (world, heightmap)
}

pub fn clamp_map<T: Into<f64>>(input: Range<f64>, output: Range<f64>, v: T) -> f64 {
    let mut normalized = (v.into() - input.start) / (input.end - input.start);

    normalized = normalized.min(1.0).max(0.0);

    (normalized * (output.end - output.start)) + output.start
}

mod tests {
    use crate::game::generation::phase2::clamp_map;

    #[test]
    fn test_clamp_map() {
        assert_eq!(clamp_map(0.0..1.0, 0.0..10.0, 0.5), 5.0);
        assert_eq!(clamp_map(0.5..1.0, -10.0..10.0, 0.5), -10.0);
    }
}