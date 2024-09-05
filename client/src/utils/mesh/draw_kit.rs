use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_WIND_STRENGTH};
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use nalgebra::Vector3;
use rc_shared::block::face::Face;
use rc_shared::chunk::LightingColor;

/// Stores all objects allowing for more ergonomic drawing of objects
pub struct DrawKit {
    pub positions: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uv_coordinates: Vec<[f32; 2]>,
    pub lighting: Vec<[f32; 4]>,
    pub wind_strength: Option<Vec<f32>>,
}

impl DrawKit {
    pub fn new() -> DrawKit {
        // TODO: Create with estimate on capacity
        DrawKit {
            positions: Vec::with_capacity(1000),
            indices: Vec::with_capacity(1000),
            normals: Vec::with_capacity(1000),
            uv_coordinates: Vec::with_capacity(1000),
            lighting: Vec::with_capacity(1000),
            wind_strength: None,
        }
    }

    pub fn with_wind_strength(mut self) -> Self {
        self.wind_strength = Some(vec![]);
        self
    }

    pub fn draw_face(
        &mut self,
        position: Vector3<f32>,
        face: &Face,
        color: LightingColor
    ) {
        let center = (face.top_right + face.bottom_left) / 2.0;

        let bottom_right = center + (center - face.top_left);

        let indices_index = self.positions.len() as u32;

        let pos = [
            position + face.top_left,
            position + face.top_right,
            position + face.bottom_left,
            position + bottom_right,
        ];

        for pos in pos {
            self.positions.push([pos.x, pos.y, pos.z]);
            self.normals
                .push([face.normal.x, face.normal.y, face.normal.z]);

            self.lighting.push([
                color[0] as f32 / 255.0,
                color[1] as f32 / 255.0,
                color[2] as f32 / 255.0,
                color[3] as f32 / 255.0,
            ]);
        }

        // TODO: Push multiple at a time
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

        if let Some(wind_strengths) = &mut self.wind_strength {
            // Default to no strength
            let face_wind = face.wind_strengths.as_ref().unwrap_or(&[0.0; 4]);

            wind_strengths.push(face_wind[0]);
            wind_strengths.push(face_wind[1]);
            wind_strengths.push(face_wind[2]);
            wind_strengths.push(face_wind[3]);
        }
    }

    pub fn apply_mesh(mut self, mesh: &mut Mesh) {
        mesh.insert_indices(Indices::U32(self.indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uv_coordinates);

        mesh.insert_attribute(
            ATTRIBUTE_LIGHTING_COLOR,
            VertexAttributeValues::Float32x4(self.lighting),
        );

        if let Some(wind_strength) = self.wind_strength.take() {
            mesh.insert_attribute(
                ATTRIBUTE_WIND_STRENGTH,
                VertexAttributeValues::Float32(wind_strength),
            );
        }
    }
}
