use bevy::asset::Assets;
use bevy::prelude::{Commands, Cuboid, EventReader, info, Mesh, PbrBundle, ResMut, Transform, Vec3};
use bevy::utils::default;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use crate::systems::chunk::ChunkSystem;

pub fn receive_column_updates(
    mut events: EventReader<ReceivePacket>,
    mut data: ResMut<ChunkSystem>,
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>
) {
    for packet in events.read() {
        let Protocol::ChunkColumnUpdate(update) = packet.0 else {
            continue
        };

        data.chunk_columns.insert(update.position, update.data.clone());

        // let pos = update.position * 16;
        //
        // for x in 0..16 {
        //     for z in 0..16_i32 {
        //
        //         let Some(y) = update.data.skylight_level[x as usize][z as usize] else {
        //             continue
        //         };
        //
        //         commands.spawn((
        //             PbrBundle {
        //                 transform: Transform::from_translation(Vec3::new((pos.x + x) as f32 + 0.5, y as f32 + 0.5, (pos.y + z) as f32 + 0.5)),
        //                 mesh: meshes.add(Mesh::from(Cuboid::from_length(0.3))),
        //                 ..default()
        //             }
        //         ));
        //     }
        // }

    }
}