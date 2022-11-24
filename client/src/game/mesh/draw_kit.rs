


use crate::game::mesh::face::Face;
use nalgebra::{Vector3};

/// Stores all objects allowing for more ergonomic drawing of objects
pub struct DrawKit<'a> {
    pub positions: &'a mut Vec<[f32; 3]>,
    pub indices: &'a mut Vec<u32>,
    pub normals: &'a mut Vec<[f32; 3]>,
    pub uv_coordinates: &'a mut Vec<[f32; 2]>,
}

impl DrawKit<'_> {
    /// Draws a set of data to buffers
    pub fn draw(
        &mut self,
        position: [f32; 3],
        indice: u32,
        normal: [f32; 3],
        uv_coordinate: [f32; 2],
    ) {
        self.positions.push(position);
        self.indices.push(indice);
        self.normals.push(normal);
        self.uv_coordinates.push(uv_coordinate);
    }

    pub fn draw_face(&mut self, position: Vector3<f32>, face: &Face) {
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
