use crate::entity::player::{Player, PlayerEntity};
use crate::game::game_state::GameState;
use crate::game::physics::PhysicsObject;
use crate::services::networking_service::send_system::{NetworkingSendSystem, SendNetworkPacket};
use nalgebra::Vector3;
use rc_network::protocol::packet::clientbound::player::position_rotation::PlayerPositionRotationPacket;
use rc_network::protocol::packet::serverbound::ServerBoundPacketData;
use specs::{Entities, Read, ReadStorage, System, WriteStorage};
use std::f32::consts::PI;

/// Updates the server when the player position or rotation changes
pub struct PlayerNetworkingSystem {
    pos: Vector3<f32>,
    rot: [f32; 2],
}

impl Default for PlayerNetworkingSystem {
    fn default() -> Self {
        PlayerNetworkingSystem {
            pos: Vector3::zeros(),
            rot: [0.0; 2],
        }
    }
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
        // TODO: Re-enable
        return;
        let player_physics = physics_objects.get(player.0).unwrap();

        if game_state.player.rot != self.rot {
            let packet =
                rc_network::protocol::packet::serverbound::player::rotation::PlayerRotationPacket {
                    yaw: (game_state.player.rot[0] / (PI * 2.0)) * 360.0,
                    pitch: (game_state.player.rot[1] / (PI * 2.0)) * 360.0,
                    on_ground: true,
                };

            entities
                .build_entity()
                .with(
                    SendNetworkPacket(ServerBoundPacketData::PlayerRotation(packet)),
                    &mut send_packets,
                )
                .build();

            self.rot = game_state.player.rot;
        }

        if player_physics.position != self.pos {
            let packet =
                rc_network::protocol::packet::serverbound::player::position::PlayerPositionPacket {
                    x: player_physics.position.x as f64,
                    y: player_physics.position.y as f64,
                    z: player_physics.position.z as f64,
                    on_ground: true,
                };

            entities
                .build_entity()
                .with(
                    SendNetworkPacket(ServerBoundPacketData::PlayerPosition(packet)),
                    &mut send_packets,
                )
                .build();

            self.pos = player_physics.position;
        }
    }
}
