use crate::helpers::to_bevy_vec3;
use crate::{info, Camera, Changed, Quat, Query, ResMut, Transform, Vec3, With};
use bevy_testing_protocol::channels::Channels;
use bevy_testing_protocol::protocol::serverbound::player_move::PlayerMove;
use bevy_testing_protocol::protocol::serverbound::player_rotate::PlayerRotate;
use bevy_testing_protocol::protocol::Protocol;
use naia_bevy_client::Client;
use nalgebra::Vector3;

const MIN_LOCATION_CHANGE_SYNC: f32 = 0.1;

pub struct LastNetworkTranslationSync(pub Vec3);
pub struct LastNetworkRotationSync(pub Quat);

pub fn network_location_sync(
    mut client: Client<Protocol, Channels>,
    query: Query<&Transform, (With<Camera>, Changed<Transform>)>,
    mut translation: ResMut<LastNetworkTranslationSync>,
    mut rotation: ResMut<LastNetworkRotationSync>,
) {
    if query.is_empty() {
        return;
    }

    let transform = query.single();

    let translation_diff = transform.translation.distance(translation.0);

    if translation_diff > MIN_LOCATION_CHANGE_SYNC {
        client.send_message(
            Channels::PlayerCommand,
            &PlayerMove::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
        );
        translation.0 = transform.translation;
    }

    let rotation_diff = (transform.rotation.x - rotation.0.x).abs()
        + (transform.rotation.y - rotation.0.y).abs()
        + (transform.rotation.z - rotation.0.z).abs()
        + (transform.rotation.w - rotation.0.w).abs();

    if rotation_diff > MIN_LOCATION_CHANGE_SYNC {
        client.send_message(
            Channels::PlayerCommand,
            &PlayerRotate::new(
                transform.rotation.x,
                transform.rotation.y,
                transform.rotation.z,
                transform.rotation.w,
            ),
        );
        rotation.0 = transform.rotation;
    }
}
