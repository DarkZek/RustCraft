use bevy::prelude::{Mesh, warn};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use nalgebra::Vector3;
use rc_shared::block::BlockStates;
use rc_shared::chunk::LightingColor;
use rc_shared::item::ItemStates;
use rc_shared::viewable_direction::{ViewableDirectionBitMap};
use crate::game::block::Draw;
use crate::utils::mesh::draw_kit::DrawKit;

const ITEM_SCALING_FACTOR: f32 = 4.0;

pub fn generate_item_mesh(
    identifier: &str,
    item_states: &ItemStates
) -> Mesh {

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let (_, item) = item_states.get_by_id(identifier).unwrap();

    if let Some(block) = item.block_definition_index {
        let block_definition = BlockStates::get_definition_by_index(block).unwrap();

        // Get variant 0
        let visual_block = block_definition.draw(0);

        visual_block.draw(
            Vector3::new(-0.5, 0.0, -0.5),
            ViewableDirectionBitMap::FULL,
            [LightingColor::full(); 6],
            &mut draw_kit
        );
    } else {
        // TODO: Handle more gracefully by spawning an empty mesh
        panic!("Block state not set for item {}", item.identifier);
    }

    for pos in &mut draw_kit.positions {
        pos[0] /= ITEM_SCALING_FACTOR;
        pos[1] /= ITEM_SCALING_FACTOR;
        pos[2] /= ITEM_SCALING_FACTOR;
    }

    draw_kit.apply_mesh(&mut mesh);

    mesh
}