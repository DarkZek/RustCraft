use crate::services::physics::PhysicsObject;
use bevy::prelude::*;

/// Syncs all of the changed PhysicsObjects to the transform
pub fn physics_sync(mut query: Query<(&mut Transform, &PhysicsObject), Changed<PhysicsObject>>) {
    for (mut transform, object) in query.iter_mut() {
        transform.translation = Vec3::new(object.position.x, object.position.y, object.position.z);
    }
}
