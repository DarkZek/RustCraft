use crate::game::update::BlockUpdateEvent;
use crate::helpers::global_to_local_position;
use crate::WorldData;
use bevy::prelude::{EventReader, ResMut};
use nalgebra::Vector3;
use rc_shared::viewable_direction::{ViewableDirection};
use rc_shared::CHUNK_SIZE;

pub fn generate_links(
    mut update_event: EventReader<BlockUpdateEvent>,
    mut chunks: ResMut<WorldData>,
) {
    for event in update_event.read() {
        // Fetch chunk info
        let (chunk_pos, local_pos) = global_to_local_position(event.pos);
        let chunk = chunks.chunks.get_mut(&chunk_pos).unwrap();

        // The index at which the pipes states start
        let pipes_index_start = 10_u32; // TODO: Fetch this dynamically
        let pipes_states_len = 2_u32.pow(6);

        // If it was a pipe
        if chunk.world.get(local_pos) < pipes_index_start
            || chunk.world.get(local_pos)
                >= pipes_index_start + pipes_states_len
        {
            continue;
        }

        // Directional bitmap of pipe directions, zeros for non pipes
        let mut pipe_data = [[[ViewableDirection(0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        // Loop over a chunk
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if chunk.world.get(Vector3::new(x, y, z)) >= pipes_index_start
                        && (chunk.world.get(Vector3::new(x, y, z))) < pipes_index_start + pipes_states_len
                    {
                        pipe_data[x][y][z] = ViewableDirection(
                            (chunk.world.get(Vector3::new(x, y, z)) - pipes_index_start) as u8,
                        );
                    }
                }
            }
        }

        let nodes = vec![];

        // Graph shit here
        let mut debugging = 0;
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if pipe_data[x][y][z].0 != 0 {
                        debugging += 1;
                    }
                }
            }
        }

        println!("Pipes in chunk {:?}", debugging);

        let network = PipeNetwork { nodes };

        // chunk
        //     .metadata
        //     .insert("pipe_network".to_string(), Box::new(network));
    }
}

pub struct PipeNetwork {
    nodes: Vec<PipeNode>,
}

pub struct PipeNode {
    position: Vector3<f32>,
    connections: Vec<usize>,
}
