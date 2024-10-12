use crate::game::world::data::WorldData;

use crate::transport::TransportSystem;
use bevy::app::App;
use bevy::prelude::{Event, EventReader, EventWriter, Plugin, Res, ResMut, Update};
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::block::BlockStates;
use rc_shared::chunk::GlobalBlockPosition;
use rc_shared::viewable_direction::BLOCK_SIDES;

/// Eventually turn this into a modular block update system

#[derive(Event)]
pub struct BlockUpdateEvent {
    pub pos: GlobalBlockPosition,
    pub block_id: u32
}

/// A block 'poke' is an event that happens when the context around a block changes so it may need to change its own state
#[derive(Event)]
pub struct BlockPokeEvent {
    pub pos: GlobalBlockPosition
}

pub struct BlockUpdatePlugin;

impl Plugin for BlockUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockUpdateEvent>()
            .add_event::<BlockPokeEvent>()
            .add_systems(Update, (update_block, do_pipes_temp));
    }
}

fn update_block(
    mut world_data: ResMut<WorldData>,
    mut update_event: EventReader<BlockUpdateEvent>,
    clients: Res<TransportSystem>,
    block_states: Res<BlockStates>,
) {
    for event in update_event.read() {
        let block_id = world_data.get_block_id(event.pos).unwrap();

        // Update chunk column
        world_data.update_column_pos(event.pos, event.block_id);
    }
}

fn do_pipes_temp(
    mut world_data: ResMut<WorldData>,
    mut update_event: EventReader<BlockPokeEvent>,
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
