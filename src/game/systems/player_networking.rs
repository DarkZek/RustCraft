use crate::entity::player::{Player, PlayerEntity};
use crate::game::game_state::GameState;
use crate::game::physics::PhysicsObject;
use crate::services::networking_service::send_system::{NetworkingSendSystem, SendNetworkPacket};
use nalgebra::Vector3;
use rc_network::protocol::packet::clientbound::player::position_rotation::PlayerPositionRotationPacket;
use rc_network::protocol::packet::serverbound::ServerBoundPacketData;
use specs::{Entities, Read, ReadStorage, System, WriteStorage};

/// Updates the server when the player position or rotation changes
pub struct PlayerNetworkingSystem {
    pos: Vector3<f32>,
    rot: [f32; 2],
}

impl<'a> System<'a> for PlayerNetworkingSystem {
    type SystemData = (
        WriteStorage<'a, SendNetworkPacket>,
        ReadStorage<'a, PhysicsObject>,
        Read<'a, GameState>,
        Read<'a, PlayerEntity>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut send_packets, physics_objects, game_state, player, mut entities): Self::SystemData,
    ) {
        let player_physics = physics_objects.get(player.0).unwrap();

        if game_state.player.rot != self.rot {
            let packet =
                rc_network::protocol::packet::serverbound::player::rotation::PlayerRotationPacket {
                    yaw: 0.0,
                    pitch: 0.0,
                    on_ground: false,
                };

            entities.build_entity().with(
                SendNetworkPacket(ServerBoundPacketData::PlayerRotation(packet)),
                &mut send_packets,
            );
        }
    }
}
