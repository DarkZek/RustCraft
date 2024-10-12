use bevy::color::palettes::tailwind::BLUE_300;
use bevy::core_pipeline::bloom::BloomSettings;
use crate::game::player::Player;
use bevy::core_pipeline::core_3d::Camera3dDepthLoadOp;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::pbr::ScreenSpaceAmbientOcclusionBundle;
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy::render::view::GpuCulling;
use crate::systems::debugging::DebuggingInfo;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_player_sync);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    // Spawn camera
    let mut camera = commands
        .spawn(Camera3dBundle {
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
        });

    camera.insert(MainCamera);

    camera
        .insert(BloomSettings::default())
        .insert(GpuCulling);

    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(ScreenSpaceAmbientOcclusionBundle::default());

    #[cfg(not(target_arch = "wasm32"))]
    camera.insert(TemporalAntiAliasBundle::default());
}

fn camera_player_sync(
    mut query: ParamSet<(
        Query<&mut Transform, (With<Transform>, With<MainCamera>)>,
        Query<&mut Transform, (With<Player>, Changed<Transform>)>,
    )>,
    debugging: Res<DebuggingInfo>
) {
    if query.p0().is_empty() || query.p1().is_empty() {
        return;
    }

    {
        // Update rotation
        let camera_rotation = query.p0().single().rotation;

        let mut player_query = query.p1();
        let mut player = player_query.single_mut();

        player.rotation = camera_rotation;
    }

    if !debugging.freecam {
        // Update position
        let player_position = query.p1().single().translation;

        let mut camera_query = query.p0();
        let mut camera = camera_query.single_mut();

        camera.translation = player_position + Vec3::new(0.0, 1.70, 0.0);
    }
}
