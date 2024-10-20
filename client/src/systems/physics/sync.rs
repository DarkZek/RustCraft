use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::{Quaternion, Vector3};
use rc_shared::helpers::{from_bevy_vec3, to_bevy_vec3};
use crate::game::game_object::player::PlayerGameObject;
use crate::game::game_object::Rotatable;

/// Syncs all of the changed PhysicsObjects to the transform
pub fn physics_location_sync(
    mut query: Query<(&mut Transform, &PhysicsObject)>,
    fixed_time: Res<Time<Fixed>>,
) {
    let overstep = fixed_time.overstep_fraction();

    for (mut transform, object) in query.iter_mut() {
        let before: Vector3<f32> = object.previous_position;
        let after: Vector3<f32> = object.position;

        let current = from_bevy_vec3(transform.translation);

        // If we're already in the desired state
        if after == current {
            continue;
        }

        let translation = before.lerp(&after, overstep);

        transform.translation = to_bevy_vec3(translation);
    }
}

/// Syncs all of the changed PhysicsObjects to the transform
pub fn physics_rotation_sync(
    game_object_data: Query<&PlayerGameObject>,
    mut transforms: Query<&mut Transform>,
    mut query: Query<(Entity, &PhysicsObject)>,
    fixed_time: Res<Time<Fixed>>,
) {
    let overstep = fixed_time.overstep_fraction();

    for (entity, physics_object) in query.iter_mut() {
        let before: Quaternion<f32> = physics_object.previous_rotation;
        let after: Quaternion<f32> = physics_object.rotation;

        // TODO: Check if position is already achieved

        let rotation = before.lerp(&after, overstep);

        let quat: Quat = rotation.into();
        let (x, y, _) = quat.to_euler(EulerRot::YXZ);

        if let Ok(player) = game_object_data.get(entity) {
            player.rotate(x, y, &mut transforms);
        } else {
            transforms.get_mut(entity).unwrap().rotation = quat;
        }
    }
}

pub fn update_last_position(
    mut query: Query<(&mut PhysicsObject)>,
) {
    for mut object in query.iter_mut() {
        object.previous_position = object.position;
        object.previous_rotation = object.rotation;
    }
}
