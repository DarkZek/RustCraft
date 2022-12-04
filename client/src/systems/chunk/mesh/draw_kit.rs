use crate::helpers::MAX_LIGHT_VALUE;
use crate::systems::chunk::data::LightingColor;
use crate::systems::chunk::mesh::face::Face;
use bevy::render::mesh::MeshVertexAttribute;
use nalgebra::Vector3;

/// Stores all objects allowing for more ergonomic drawing of objects
pub struct DrawKit<'a> {
    pub positions: &'a mut Vec<[f32; 3]>,
    pub indices: &'a mut Vec<u32>,
    pub normals: &'a mut Vec<[f32; 3]>,
    pub uv_coordinates: &'a mut Vec<[f32; 2]>,
    pub lighting: &'a mut Vec<[f32; 4]>,
}

impl DrawKit<'_> {
    pub fn draw_face(&mut self, position: Vector3<f32>, face: &Face, mut color: LightingColor) {
        // Light falloff
        color[0] = (color[0] as u32 * color[3] as u32 / MAX_LIGHT_VALUE as u32) as u8;
        color[1] = (color[1] as u32 * color[3] as u32 / MAX_LIGHT_VALUE as u32) as u8;
        color[2] = (color[2] as u32 * color[3] as u32 / MAX_LIGHT_VALUE as u32) as u8;

        let center = (face.top_right + face.bottom_left) / 2.0;

        let bottom_right = center + (center - face.top_left);

        let indices_index = self.positions.len() as u32;

        let mut pos = [
            position + face.top_left,
            position + face.top_right,
            position + face.bottom_left,
            position + bottom_right,
        ];

        let color = [
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        ];

        for pos in pos {
            self.positions.push([pos.x, pos.y, pos.z]);
            self.normals
                .push([face.normal.x, face.normal.y, face.normal.z]);
            self.lighting.push(color);
        }

        self.uv_coordinates
            .push([face.texture.u_min, face.texture.v_max]);
        self.uv_coordinates
            .push([face.texture.u_min, face.texture.v_min]);
        self.uv_coordinates
            .push([face.texture.u_max, face.texture.v_max]);
        self.uv_coordinates
            .push([face.texture.u_max, face.texture.v_min]);

        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
    }
}
