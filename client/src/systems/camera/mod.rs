use crate::game::entity::Entity;
use crate::game::player::Player;
use crate::systems::physics::aabb::Aabb;
use crate::systems::physics::PhysicsObject;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::core_pipeline::core_3d::{Camera3dDepthLoadOp, Camera3dDepthTextureUsage};
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use nalgebra::Vector3;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_player_sync);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut player_physics = PhysicsObject::new(
        Vector3::new(0.0, 40.0, 0.0),
        Aabb::new(
            Vector3::new(-0.35, -1.7, -0.35),
            Vector3::new(0.7, 1.85, 0.7),
        ),
    );

    // Enable gravity for local player
    player_physics.gravity = true;

    let start_transform = Transform::from_translation(Vec3::new(
        player_physics.position.x,
        player_physics.position.y,
        player_physics.position.z,
    ));

    // Spawn camera
    commands
        .spawn(Camera3dBundle {
            transform: start_transform,
            camera_3d: Camera3d {
                /// The clear color operation to perform for the main 3d pass.
                clear_color: ClearColorConfig::Custom(Color::rgba(0.7137, 0.7803, 0.8784, 1.0)),
                depth_load_op: Camera3dDepthLoadOp::Clear(0.0),
                depth_texture_usages: TextureUsages::RENDER_ATTACHMENT.into(),
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 3.0,
                near: 0.1,
                far: 1000.0,
                aspect_ratio: 1.0,
            }),
            ..default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle::default());
    //.insert(TemporalAntiAliasBundle::default());

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
