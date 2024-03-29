use crate::game::world::data::WorldData;

use crate::transport::TransportSystem;
use bevy::app::App;
use bevy::prelude::{Event, EventReader, EventWriter, Plugin, Res, ResMut, Update};
use nalgebra::Vector3;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::viewable_direction::BLOCK_SIDES;

/// Eventually turn this into a modular block update system

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub pos: Vector3<i32>,
}

pub struct BlockUpdatePlugin;

impl Plugin for BlockUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockUpdateEvent>()
            .add_systems(Update, update_block);
    }
}

fn update_block(
    mut world_data: ResMut<WorldData>,
    mut update_event: EventReader<BlockUpdateEvent>,
    mut send_packet: EventWriter<SendPacket>,
    clients: Res<TransportSystem>,
    block_states: Res<BlockStates>,
) {
    for event in update_event.read() {
        let block_id = world_data.get_block_id(event.pos).unwrap();

        let block = block_states.get_block(block_id as usize);

        // Pipe check (TEMP)
        if block.identifier != "mcv3::Pipe" {
            continue;
        }

        // Get surrounding blocks
        let mut val = 0;
        for dir in 0..6 {
            let block_id = world_data
                .get_block_id(event.pos + BLOCK_SIDES[dir])
                .unwrap();

            if block_id >= 9 {
                // Also include type spawner
                val |= 0b100000 >> dir;
            }
        }

        world_data.set_block_id(event.pos, val + 10);

        // Notify all clients
        for (uid, _) in &clients.clients {
            send_packet.send(SendPacket(
                Protocol::BlockUpdate(
                    rc_networking::protocol::clientbound::block_update::BlockUpdate::new(
                        val + 10,
                        event.pos.x,
                        event.pos.y,
                        event.pos.z,
                    ),
                ),
                *uid,
            ));
        }
    }
}
