use crate::game::entity::Entity;
use crate::game::player::Player;
use crate::services::physics::aabb::Aabb;
use crate::services::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::Vector3;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_player_sync);
    }
}

fn setup_camera(mut commands: Commands) {
    let player_physics = PhysicsObject::new(
        Vector3::new(0.0, 40.0, 0.0),
        vec![Aabb::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 1.0),
        )],
    );
    let start_transform = Transform::from_translation(Vec3::new(
        player_physics.position.x,
        player_physics.position.y,
        player_physics.position.z,
    ));

    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: start_transform,
        ..default()
    });

    // Spawn player
    // Todo: Move this elsewhere
    commands
        .spawn(start_transform)
        .insert(player_physics)
        .insert(Entity)
        .insert(Player::new());
}

fn camera_player_sync(
    mut query: ParamSet<(
        Query<&mut Transform, (With<Transform>, &Camera)>,
        Query<&Transform, (With<Player>, Changed<Transform>)>,
    )>,
) {
    if query.p0().is_empty() {
        return;
    }
    if query.p1().is_empty() {
        return;
    }

    let player: Transform = query.p1().single().clone();

    let mut camera_query = query.p0();

    let mut camera = camera_query.single_mut();

    *camera = player;
}
