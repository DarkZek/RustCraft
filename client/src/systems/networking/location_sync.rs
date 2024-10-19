use bevy::prelude::*;
use nalgebra::{Quaternion, Vector3};
use rc_shared::constants::UserId;
use rc_networking::protocol::serverbound::player_move::PlayerMove;
use rc_networking::protocol::serverbound::player_rotate::PlayerRotate;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use crate::game::player::Player;
use crate::systems::physics::PhysicsObject;

const MIN_LOCATION_CHANGE_SYNC: f32 = 0.025;

#[derive(Resource)]
pub struct LastNetworkTranslationSync(pub Vector3<f32>);
#[derive(Resource)]
pub struct LastNetworkRotationSync(pub Quat);

pub fn network_location_sync(
    physics_query: Query<&PhysicsObject, (With<Player>, Changed<PhysicsObject>)>,
    transform_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut translation: ResMut<LastNetworkTranslationSync>,
    mut rotation: ResMut<LastNetworkRotationSync>,
    mut networking: EventWriter<SendPacket>,
) {
    if let Ok(physics_object) = physics_query.get_single() {
        let translation_diff = (physics_object.position - translation.0).magnitude();

        if translation_diff > MIN_LOCATION_CHANGE_SYNC {
            networking.send(SendPacket(
                Protocol::PlayerMove(PlayerMove::new(
                    physics_object.position.x,
                    physics_object.position.y,
                    physics_object.position.z,
                )),
                UserId(0),
            ));
            translation.0 = physics_object.position;
        }
    }

    if let Ok(transform) = transform_query.get_single() {
        let rotation_diff = (transform.rotation.x - rotation.0.x).abs()
            + (transform.rotation.y - rotation.0.y).abs()
            + (transform.rotation.z - rotation.0.z).abs()
            + (transform.rotation.w - rotation.0.w).abs();

        if rotation_diff > MIN_LOCATION_CHANGE_SYNC {
            networking.send(SendPacket(
                Protocol::PlayerRotate(PlayerRotate {
                    x: transform.rotation.x,
                    y: transform.rotation.y,
                    z: transform.rotation.z,
                    w: transform.rotation.w,
                }),
                UserId(0),
            ));
            rotation.0 = transform.rotation;
        }
    }
}
