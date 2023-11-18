use crate::systems::chunk::builder::ATTRIBUTE_LIGHTING_COLOR;
use crate::systems::chunk::data::LightingColor;
use crate::systems::chunk::mesh::face::Face;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use nalgebra::Vector3;

/// Stores all objects allowing for more ergonomic drawing of objects
pub struct DrawKit {
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uv_coordinates: Vec<[f32; 2]>,
    pub lighting: Vec<[f32; 4]>,
}

impl DrawKit {
    pub fn new() -> DrawKit {
        DrawKit {
            positions: vec![],
            indices: vec![],
            normals: vec![],
            uv_coordinates: vec![],
            lighting: vec![],
        }
    }

    pub fn draw_face(&mut self, position: Vector3<f32>, face: &Face, color: LightingColor) {
        let center = (face.top_right + face.bottom_left) / 2.0;

        let bottom_right = center + (center - face.top_left);

        let indices_index = self.positions.len() as u32;

        let pos = [
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

    pub fn apply_mesh(self, mesh: &mut Mesh) {
        mesh.set_indices(Some(Indices::U32(self.indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uv_coordinates);
        mesh.insert_attribute(
            ATTRIBUTE_LIGHTING_COLOR,
            VertexAttributeValues::Float32x4(self.lighting),
        );
    }
}
