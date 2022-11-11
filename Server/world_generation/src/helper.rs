use std::ops::{Add, Div, Sub};
use crate::constants::CHUNK_SIZE;

pub fn calculate_global_pos(chunk: &[i32; 2], local: [usize; 2]) -> [i32; 2] {
    [(chunk[0] * CHUNK_SIZE as i32) + local[0] as i32, (chunk[1] * CHUNK_SIZE as i32) + local[1] as i32]
}

pub trait Map {
    fn map(self, in_min: Self, in_max: Self, out_min: Self, out_max: Self) -> Self;
}

impl Map for u32 {
    fn map(self, in_min: Self, in_max: Self, out_min: Self, out_max: Self) -> Self {
        let in_len = in_max - in_min;
        let out_len = out_max - out_min;

        ((self - in_min) * (out_len/in_len)) + out_min
    }
}

impl Map for f32 {
    fn map(self, in_min: Self, in_max: Self, out_min: Self, out_max: Self) -> Self {
        let in_len = in_max - in_min;
        let out_len = out_max - out_min;

        ((self - in_min) * (out_len/in_len)) + out_min
    }
}