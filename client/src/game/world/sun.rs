use crate::game::player::Player;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::render::primitives::Plane;
use std::f32::consts::PI;

#[derive(Default)]
pub struct SunData {
    sprite: Option<Entity>,
    light: Option<Entity>,
}

pub fn setup_sun(
    mut commands: Commands,
    query: Query<Entity, With<Camera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sundata: Local<SunData>,
) {
    // Camera entity
    let entity = query.single();

    // TODO: Spawn this attached to the camera so it moves around with it
    sundata.light = Some(commands.entity(entity).add_children(|child| {
        child
            .spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(1., 1., 1.),
                    illuminance: 50000.0,
                    shadow_projection: OrthographicProjection {
                        left: -40.0,
                        right: 40.0,
                        bottom: -40.0,
                        top: 40.0,
                        near: -20.0,
                        far: 50.0,
                        ..default()
                    },
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_euler(
                    EulerRot::ZYX,
                    0.0,
                    PI / 6.,
                    -PI / 5.,
                )),
                ..default()
            })
            .id()
    }));

    sundata.sprite = Some(commands.entity(entity).add_children(|child| {
        child
            .spawn(PbrBundle {
                //mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(10.0, 10.0)))),
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                material: Default::default(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 8.0)),
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            })
            .id()
    }));
}
