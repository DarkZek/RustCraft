use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::Vector3;

pub fn physics_tick(mut query: Query<&mut PhysicsObject>, time: Res<Time>) {
    for mut object in query.iter_mut() {
        object.previous_position = object.position;
        object.position = object.position + (object.velocity * time.delta_seconds());
        object.velocity *= 0.92;

        // Stop when going slow enough to save computation
        if object.position.norm() < 0.1 {
            object.velocity = Vector3::zeros();
        }
    }
}
