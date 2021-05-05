use crate::helpers::Lerp;

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
