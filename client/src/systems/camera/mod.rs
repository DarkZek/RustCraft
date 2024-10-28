use bevy::core_pipeline::bloom::BloomSettings;
use crate::game::player::Player;
use bevy::core_pipeline::core_3d::Camera3dDepthLoadOp;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy::render::view::GpuCulling;
use rc_shared::helpers::to_bevy_vec3;
use crate::systems::debugging::DebuggingInfo;
use crate::systems::physics::PhysicsObject;
use crate::systems::post_processing::settings::PostProcessSettings;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, (camera_player_location_sync, camera_player_rotation_sync));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    // Spawn camera
    let mut camera = commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    ..default()
                },
                camera_3d: Camera3d {
                    depth_load_op: Camera3dDepthLoadOp::Clear(0.0),
                    depth_texture_usages: TextureUsages::RENDER_ATTACHMENT.into(),
                    ..default()
                },
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::PI / 3.0,
                    near: 0.1,
                    far: 1000.0,
                    aspect_ratio: 1.0,
                }),
                ..default()
            },
            PostProcessSettings {
                intensity: 0.02,
                ..default()
            }
        ));

    camera.insert(MainCamera);

    camera
        .insert(BloomSettings::default())
        .insert(GpuCulling);

    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(ScreenSpaceAmbientOcclusionBundle::default());

    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(TemporalAntiAliasBundle::default());
}

// Take the location from the `Player` and update the camera's position
fn camera_player_location_sync(
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>
    )>,
    debugging: Res<DebuggingInfo>
) {
    let Ok(player_position) = query.p0().get_single().map(|transform| transform.translation) else {
        return;
    };
    let mut camera_query = query.p1();
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    if debugging.freecam {
       return
    }

    camera_transform.translation = player_position + Vec3::new(0.0, 1.70, 0.0);
}

// Take the rotation from the camera and update the player
fn camera_player_rotation_sync(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut player_query: Query<&mut PhysicsObject, (With<Player>)>
) {
    let Ok(mut player_physics) = player_query.get_single_mut() else {
        return;
    };
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    player_physics.rotation = camera_transform.rotation.into();
}
