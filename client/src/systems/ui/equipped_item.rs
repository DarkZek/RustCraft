use bevy::pbr::NotShadowCaster;
use bevy::prelude::{Assets, Camera, Camera3dBundle, ClearColorConfig, Commands, default, Entity, Handle, MaterialMeshBundle, Mesh, PerspectiveProjection, Query, Res, ResMut, Resource, Transform};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use rc_shared::block::BlockStates;
use rc_shared::item::ItemStates;
use crate::game::game_object::mesh::generate_item_mesh;
use crate::game::inventory::Inventory;
use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_WIND_STRENGTH};

/// Used by the view model camera and the player's equipped item.
/// The light sources belong to both layers
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Resource)]
pub struct EquippedItemData {
    equipped_identifier: Option<String>,
    mesh_entity: Entity
}

pub fn setup_equipped_item(
    mut commands: Commands,
    asset_service: Res<AssetService>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // Bump the order to render on top of the world model.
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            projection: PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                ..default()
            }
                .into(),
            ..default()
        },
        // Only render objects belonging to the view model.
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
    ));

    let mut empty_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    empty_mesh.insert_attribute(ATTRIBUTE_WIND_STRENGTH, vec![] as Vec<f32>);
    empty_mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, vec![] as Vec<[f32; 4]>);

    // Spawn the player's right arm.
    let mesh_entity = commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(empty_mesh),
            material: asset_service.translucent_texture_atlas_material.clone(),
            transform: Transform::from_xyz(0.3, -0.34, -0.4),
            ..default()
        },
        // Ensure the arm is only rendered by the view model camera.
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        // The arm is free-floating, so shadows would look weird.
        NotShadowCaster,
    )).id();

    commands.insert_resource(EquippedItemData {
        equipped_identifier: None,
        mesh_entity,
    });
}

pub fn update_equipped_item_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Handle<Mesh>>,
    mut data: ResMut<EquippedItemData>,
    block_states: Res<BlockStates>,
    item_states: Res<ItemStates>,
    inventory: Res<Inventory>
) {

    let identifier = inventory.selected_item().map(|t| &t.item.identifier);
    if data.equipped_identifier.as_ref() == identifier {
        return;
    }

    data.equipped_identifier = identifier.map(|t| t.clone());

    let mesh = if let Some(identifier) = &data.equipped_identifier {
        generate_item_mesh(identifier, &block_states, &item_states)
    } else {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

        mesh.insert_attribute(ATTRIBUTE_WIND_STRENGTH, vec![] as Vec<f32>);
        mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, vec![] as Vec<[f32; 4]>);

        mesh
    };

    let mesh_handle = query.get(data.mesh_entity).unwrap();

    *meshes.get_mut(mesh_handle).unwrap() = mesh;
}