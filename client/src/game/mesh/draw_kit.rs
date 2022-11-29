use crate::game::mesh::face::Face;
use crate::services::chunk::data::LightingColor;
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
    pub fn draw_face(&mut self, position: Vector3<f32>, face: &Face, color: LightingColor) {
        let center = (face.top_right + face.bottom_left) / 2.0;

        let bottom_right = center + (center - face.top_left);

        let indices_index = self.positions.len() as u32;

        self.positions.push([
            position.x + face.top_left.x,
            position.y + face.top_left.y,
            position.z + face.top_left.z,
        ]);
        self.positions.push([
            position.x + face.top_right.x,
            position.y + face.top_right.y,
            position.z + face.top_right.z,
        ]);
        self.positions.push([
            position.x + face.bottom_left.x,
            position.y + face.bottom_left.y,
            position.z + face.bottom_left.z,
        ]);
        self.positions.push([
            position.x + bottom_right.x,
            position.y + bottom_right.y,
            position.z + bottom_right.z,
        ]);

        self.normals
            .push([face.normal.x, face.normal.y, face.normal.z]);
        self.normals
            .push([face.normal.x, face.normal.y, face.normal.z]);
        self.normals
            .push([face.normal.x, face.normal.y, face.normal.z]);
        self.normals
            .push([face.normal.x, face.normal.y, face.normal.z]);

        let color = [
            color[0] as f32 / 255.0,
            color[1] as f32 / 255.0,
            color[2] as f32 / 255.0,
            color[3] as f32 / 255.0,
        ];
        self.lighting.push(color);
        self.lighting.push(color);
        self.lighting.push(color);
        self.lighting.push(color);

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
