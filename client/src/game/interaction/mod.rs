use bevy::app::{App, Update};
use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::pbr::{NotShadowCaster, PbrBundle, StandardMaterial};
use bevy::prelude::{AlphaMode, Commands, Cuboid, Mesh, Plugin, ResMut, Startup, Visibility};
use bevy::utils::default;
use crate::game::interaction::destroy::{mouse_interaction_destroy, MouseInteractionResource};
use crate::game::interaction::place::mouse_interaction_place;

pub const MAX_INTERACTION_DISTANCE: f32 = 7.0;

pub mod highlight;
mod destroy;
mod place;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (mouse_interaction_destroy, mouse_interaction_place))
            .add_systems(Startup, setup_mouse_interaction)
            .insert_resource(MouseInteractionResource::default());
    }
}

pub fn setup_mouse_interaction(
    mut commands: Commands,
    mut resource: ResMut<MouseInteractionResource>,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = assets.add(Mesh::from(Cuboid::from_length(1.0)));
    let material = materials.add(StandardMaterial {
        base_color: Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.08)),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    resource.block_selection_entity = Some(commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Visibility::Hidden,
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            ..default()
        },
        NotShadowCaster
    )).id());
}