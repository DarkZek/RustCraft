use crate::helpers::Lerp;
use nalgebra::Vector2;
use std::f32::consts::PI;
use std::ops::Mul;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, PartialEq)]
pub enum Rotate {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TextureAtlasIndex {
    pub u_min: f32,
    pub u_max: f32,
    pub v_min: f32,
    pub v_max: f32,
}

impl TextureAtlasIndex {
    pub fn new(u_min: f32, u_max: f32, v_min: f32, v_max: f32) -> TextureAtlasIndex {
        TextureAtlasIndex {
            u_min,
            u_max,
            v_min,
            v_max,
        }
    }

    pub fn width(&self) -> f32 {
        (self.u_max - self.u_min).abs()
    }

    pub fn height(&self) -> f32 {
        (self.v_max - self.v_min).abs()
    }

    pub fn half_width(&self) -> f32 {
        (self.u_max - self.u_min) / 2.0
    }

    pub fn half_height(&self) -> f32 {
        (self.v_max - self.v_min) / 2.0
    }

    pub fn invert(&self) -> TextureAtlasIndex {
        TextureAtlasIndex {
            u_min: self.u_max,
            u_max: self.u_min,
            v_min: self.v_max,
            v_max: self.v_min,
        }
    }

    pub fn rotate(&self, deg: Rotate) -> TextureAtlasIndex {
        if deg == Rotate::Deg90 {
            return TextureAtlasIndex {
                u_min: self.u_max,
                u_max: self.u_min,
                v_min: self.v_min,
                v_max: self.v_max,
            };
        }

        let rot = match deg {
            Rotate::Deg90 => PI * 0.5,
            Rotate::Deg180 => PI,
            Rotate::Deg270 => PI * 1.5,
            _ => return self.clone(),
        };

        let center_u = (self.u_min + self.u_max) / 2.0;
        let center_v = (self.v_min + self.v_max) / 2.0;

        let mut p1 = Vector2::new(self.u_min, self.v_min);
        let mut p2 = Vector2::new(self.u_max, self.v_max);

        // Make relative
        p1 -= Vector2::new(center_u, center_v);
        p2 -= Vector2::new(center_u, center_v);

        let rotation_vector = nalgebra::geometry::Rotation2::new(rot);
        let rotation_matrix = rotation_vector.matrix();
        p1 = rotation_matrix.mul(&p1);
        p2 = rotation_matrix.mul(&p2);

        // Make un relative
        p1 += Vector2::new(center_u, center_v);
        p2 += Vector2::new(center_u, center_v);

        TextureAtlasIndex {
            u_min: p1.x,
            u_max: p2.x,
            v_min: p1.y,
            v_max: p2.y,
        }
    }

    pub fn local_offset(
        &self,
        u_min: Option<f32>,
        u_max: Option<f32>,
        v_min: Option<f32>,
        v_max: Option<f32>,
    ) -> TextureAtlasIndex {
        let mut atlas = self.clone();

        if u_min.is_some() {
            atlas.u_min += u_min.unwrap();
        }

        if u_max.is_some() {
            atlas.u_max += u_max.unwrap();
        }

        if v_min.is_some() {
            atlas.v_min += v_min.unwrap();
        }

        if v_max.is_some() {
            atlas.v_max += v_max.unwrap();
        }

        atlas
    }

    pub fn sub_index(&self, index: &TextureAtlasIndex) -> TextureAtlasIndex {
        TextureAtlasIndex::new(
            self.u_min.lerp(self.u_max, index.u_min),
            self.u_min.lerp(self.u_max, index.u_max),
            self.v_min.lerp(self.v_max, index.v_min),
            self.v_min.lerp(self.v_max, index.v_max),
        )
    }

    pub fn multiply(&mut self, width: f32, height: f32) {
        self.u_min *= width;
        self.u_max *= width;
        self.v_min *= height;
        self.v_max *= height;
    }

    pub fn flipped(&self) -> TextureAtlasIndex {
        let mut index = self.clone();

        let temp_min = index.u_min;
        index.u_min = index.u_max;
        index.u_max = temp_min;

        index
    }
}

impl Default for TextureAtlasIndex {
    fn default() -> Self {
        TextureAtlasIndex::new(0.0, 0.0, 0.0, 0.0)
    }
}
