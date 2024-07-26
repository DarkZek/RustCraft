use nalgebra::Vector3;
use rc_shared::block::types::Block;
use rc_shared::chunk::LightingColor;
use rc_shared::viewable_direction::{AxisAlignedDirection, ViewableDirection};
use crate::utils::mesh::draw_kit::DrawKit;

pub trait Draw {
    fn draw(
        &self,
        pos: Vector3<f32>,
        visible_map: ViewableDirection,
        light_color: Option<[LightingColor; 6]>,
        kit: &mut DrawKit,
    );
}

impl Draw for Block {
    fn draw(
        &self,
        pos: Vector3<f32>,
        visible_map: ViewableDirection,
        light_color: Option<[LightingColor; 6]>,
        kit: &mut DrawKit,
    ) {
        for face in &self.faces {
            if !visible_map.has_flag(face.direction) && face.edge {
                // Not visible from that direction and marked as an edge face, so cull
                continue;
            }

            // Get lighting color
            let color = light_color.map(|color|
                color[AxisAlignedDirection::from(face.direction) as usize]);

            kit.draw_face(pos, face, color);
        }
    }
}
