use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::math::Vec3;
use bevy::prelude::{default, Assets, Commands, Entity, Query, Res, ResMut, Resource, Transform, With, Without, LinearRgba};
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, to_bevy_vec3};
use crate::systems::camera::MainCamera;

#[derive(Resource, Default)]
pub struct HighlightData {
    polyline: Option<Entity>,
}

pub fn setup_highlights(
    mut highlight_data: ResMut<HighlightData>,
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    highlight_data.polyline = Some(
        commands
            .spawn(PolylineBundle {
                polyline: polylines.add(Polyline {
                    vertices: vec![
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, 1.0, 1.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(1.0, 0.0, 1.0),
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(1.0, 1.0, 0.0),
                        Vec3::new(0.0, 1.0, 0.0),
                        Vec3::new(0.0, 1.0, 1.0),
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(1.0, 0.0, 1.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(1.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 0.0),
                    ],
                }),
                material: polyline_materials.add(PolylineMaterial {
                    width: 2.0,
                    color: LinearRgba::new(1.0, 1.0, 1.0, 0.5),
                    perspective: false,
                    // Bias the line toward the camera so the line at the cube-plane intersection is visible
                    depth_bias: -0.0002,
                }),
                ..default()
            })
            .id(),
    );
}

pub fn mouse_highlight_interaction(
    highlight_data: ResMut<HighlightData>,
    mut translation_data: Query<&mut Transform, Without<MainCamera>>,
    camera: Query<&Transform, With<MainCamera>>,
    chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
) {
    let camera_pos = camera.get_single().unwrap();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        15.0,
        &chunks,
        &blocks,
    );

    if let Some(ray) = cast {
        translation_data
            .get_mut(highlight_data.polyline.unwrap())
            .unwrap()
            .translation = to_bevy_vec3(ray.block.cast::<f32>());
    }
}
