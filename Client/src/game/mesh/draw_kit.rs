use crate::game::viewable_direction::{ViewableDirection, ViewableDirectionBitMap};
use crate::services::asset::atlas::index::TextureAtlasIndex;

use nalgebra::{Vector2, Vector3};

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

    pub fn draw_full_block(
        &mut self,
        position: Vector3<f32>,
        visibility: ViewableDirection,
        texture: TextureAtlasIndex,
    ) {
        if visibility.0 == 0 {
            return;
        }
        if visibility.has_flag(ViewableDirectionBitMap::Top) {
            self.draw_top_face(
                position + Vector3::new(0.0, 1.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Bottom) {
            self.draw_bottom_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Front) {
            self.draw_front_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Back) {
            self.draw_back_face(
                position + Vector3::new(0.0, 0.0, 1.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Left) {
            self.draw_left_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Right) {
            self.draw_right_face(
                position + Vector3::new(1.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                texture,
            );
        }
    }

    pub fn draw_full_block_textures(
        &mut self,
        position: Vector3<f32>,
        visibility: ViewableDirection,
        textures: [TextureAtlasIndex; 6],
    ) {
        if visibility.0 == 0 {
            return;
        }
        if visibility.has_flag(ViewableDirectionBitMap::Top) {
            self.draw_top_face(
                position + Vector3::new(0.0, 1.0, 0.0),
                Vector2::new(1.0, 1.0),
                textures[0],
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Bottom) {
            self.draw_bottom_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                textures[1],
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Front) {
            self.draw_front_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                textures[2],
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Back) {
            self.draw_back_face(
                position + Vector3::new(0.0, 0.0, 1.0),
                Vector2::new(1.0, 1.0),
                textures[3],
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Left) {
            self.draw_left_face(
                position + Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                textures[4],
            );
        }
        if visibility.has_flag(ViewableDirectionBitMap::Right) {
            self.draw_right_face(
                position + Vector3::new(1.0, 0.0, 0.0),
                Vector2::new(1.0, 1.0),
                textures[5],
            );
        }
    }

    pub fn draw_top_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y, position.z + size.y]);
        self.positions
            .push([position.x + size.x, position.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y, position.z + size.y]);

        self.normals.push([0.0, 1.0, 0.0]);
        self.normals.push([0.0, 1.0, 0.0]);
        self.normals.push([0.0, 1.0, 0.0]);
        self.normals.push([0.0, 1.0, 0.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 2);
    }

    pub fn draw_bottom_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y, position.z + size.y]);
        self.positions
            .push([position.x + size.x, position.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y, position.z + size.y]);

        self.normals.push([0.0, -1.0, 0.0]);
        self.normals.push([0.0, -1.0, 0.0]);
        self.normals.push([0.0, -1.0, 0.0]);
        self.normals.push([0.0, -1.0, 0.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
    }

    pub fn draw_left_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y + size.y, position.z]);
        self.positions
            .push([position.x, position.y, position.z + size.x]);
        self.positions
            .push([position.x, position.y + size.y, position.z + size.x]);

        self.normals.push([-1.0, 0.0, 0.0]);
        self.normals.push([-1.0, 0.0, 0.0]);
        self.normals.push([-1.0, 0.0, 0.0]);
        self.normals.push([-1.0, 0.0, 0.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
    }

    pub fn draw_right_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y + size.y, position.z]);
        self.positions
            .push([position.x, position.y, position.z + size.x]);
        self.positions
            .push([position.x, position.y + size.y, position.z + size.x]);

        self.normals.push([1.0, 0.0, 0.0]);
        self.normals.push([1.0, 0.0, 0.0]);
        self.normals.push([1.0, 0.0, 0.0]);
        self.normals.push([1.0, 0.0, 0.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 2);
    }

    pub fn draw_front_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y + size.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y + size.y, position.z]);

        self.normals.push([0.0, 0.0, -1.0]);
        self.normals.push([0.0, 0.0, -1.0]);
        self.normals.push([0.0, 0.0, -1.0]);
        self.normals.push([0.0, 0.0, -1.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 2);
    }

    pub fn draw_back_face(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureAtlasIndex,
    ) {
        let indices_index = self.positions.len() as u32;

        self.positions.push([position.x, position.y, position.z]);
        self.positions
            .push([position.x, position.y + size.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y, position.z]);
        self.positions
            .push([position.x + size.x, position.y + size.y, position.z]);

        self.normals.push([0.0, 0.0, 1.0]);
        self.normals.push([0.0, 0.0, 1.0]);
        self.normals.push([0.0, 0.0, 1.0]);
        self.normals.push([0.0, 0.0, 1.0]);

        self.uv_coordinates.push([texture.u_min, texture.v_max]);
        self.uv_coordinates.push([texture.u_min, texture.v_min]);
        self.uv_coordinates.push([texture.u_max, texture.v_max]);
        self.uv_coordinates.push([texture.u_max, texture.v_min]);

        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 0);
        self.indices.push(indices_index + 2);
        self.indices.push(indices_index + 3);
        self.indices.push(indices_index + 1);
        self.indices.push(indices_index + 2);
    }
}
