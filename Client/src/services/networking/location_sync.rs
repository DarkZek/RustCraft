
use crate::services::networking::transport::packet::SendPacket;
use crate::{Camera, Changed, EventWriter, Quat, Query, ResMut, Transform, Vec3, With};


use rustcraft_protocol::protocol::serverbound::player_move::PlayerMove;
use rustcraft_protocol::protocol::serverbound::player_rotate::PlayerRotate;
use rustcraft_protocol::protocol::Protocol;

const MIN_LOCATION_CHANGE_SYNC: f32 = 0.1;

pub struct LastNetworkTranslationSync(pub Vec3);
pub struct LastNetworkRotationSync(pub Quat);

pub fn network_location_sync(
    query: Query<&Transform, (With<Camera>, Changed<Transform>)>,
    mut translation: ResMut<LastNetworkTranslationSync>,
    mut rotation: ResMut<LastNetworkRotationSync>,
    mut networking: EventWriter<SendPacket>,
) {
    if query.is_empty() {
        return;
    }

    let transform = query.single();

    let translation_diff = transform.translation.distance(translation.0);

    if translation_diff > MIN_LOCATION_CHANGE_SYNC {
        networking.send(SendPacket(Protocol::PlayerMove(PlayerMove::new(
            transform.translation.x,
            transform.translation.y,
            transform.translation.z,
        ))));
        translation.0 = transform.translation;
    }

    let rotation_diff = (transform.rotation.x - rotation.0.x).abs()
        + (transform.rotation.y - rotation.0.y).abs()
        + (transform.rotation.z - rotation.0.z).abs()
        + (transform.rotation.w - rotation.0.w).abs();

    if rotation_diff > MIN_LOCATION_CHANGE_SYNC {
        networking.send(SendPacket(Protocol::PlayerRotate(PlayerRotate {
            x: transform.rotation.x,
            y: transform.rotation.y,
            z: transform.rotation.z,
            w: transform.rotation.w,
        })));
        rotation.0 = transform.rotation;
    }
}
