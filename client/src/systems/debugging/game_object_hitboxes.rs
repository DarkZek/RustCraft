use bevy::prelude::{Gizmos, Query, Transform, With};
use nalgebra::Vector3;
use crate::game::entity::GameObject;
use crate::systems::physics::PhysicsObject;

const DISPLAY_DISTANCE: i32 = 5;

/// Get the height of the skylight heightmap of the chunk at the camera and draw cubes to show it
pub fn draw_game_object_hitboxes(
    entities: Query<(&Transform, &PhysicsObject), With<GameObject>>,
    mut gizmos: Gizmos
) {
    for (transform, physics) in entities.iter() {
        physics.collider.draw_gizmo(&mut gizmos, Vector3::new(transform.translation.x, transform.translation.y, transform.translation.z));
    }
}