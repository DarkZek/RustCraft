use crate::services::chunk::data::{ChunkData, Color};
use crate::Vec3;
use bevy::prelude::Query;
use bevy_testing_protocol::constants::CHUNK_SIZE;
use nalgebra::{Point3, Vector3};
use std::ops::Add;

/// Formats a u32 with American comma placement.
///
/// # Example
/// ```rust
/// assert_eq!(String::from("9,000,000"), format_u32(9000000).to_string());
/// ```
pub fn format_u32(mut count: u32) -> String {
    let mut msg = String::new();

    while count != 0 {
        if count / 1000 == 0 {
            msg = (count % 1000).to_string().add(msg.as_str());
        } else {
            msg = format!(",{:03}", count % 1000).add(msg.as_str());
        }

        count = count / 1000;
    }

    msg
}

pub trait Lerp {
    fn lerp(self, b: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, b: Self, t: f32) -> Self {
        ((b - self) * t) + self
    }
}

impl Lerp for u8 {
    fn lerp(self, b: Self, t: f32) -> Self {
        ((b as f32 - self as f32) * t as f32) as u8 + self
    }
}

pub fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    if t == 0.0 {
        return c1;
    }
    [
        if c1[0] < c2[0] {
            c1[0].lerp(c2[0], t)
        } else {
            c2[0].lerp(c1[0], t)
        },
        if c1[1] < c2[1] {
            c1[1].lerp(c2[1], t)
        } else {
            c2[1].lerp(c1[1], t)
        },
        if c1[2] < c2[2] {
            c1[2].lerp(c2[2], t)
        } else {
            c2[2].lerp(c1[2], t)
        },
        if c1[3] < c2[3] {
            c1[3].lerp(c2[3], t)
        } else {
            c2[3].lerp(c1[3], t)
        },
    ]
}

pub fn distance(p1: &Point3<usize>, p2: &Point3<usize>) -> u32 {
    ((p1.x as isize - p2.x as isize).abs()
        + (p1.y as isize - p2.y as isize).abs()
        + (p1.z as isize - p2.z as isize).abs())
    .abs() as u32
}

pub trait Clamp {
    fn clamp_val(self, min: Self, max: f32) -> f32;
}

impl Clamp for f32 {
    fn clamp_val(self, min: f32, max: f32) -> f32 {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}

/// Converts global coordinate axis to a local one inside the data
#[inline]
pub fn get_chunk_coords(i: i32) -> usize {
    ((i % CHUNK_SIZE as i32) + CHUNK_SIZE as i32) as usize % CHUNK_SIZE
}

#[inline]
pub fn from_bevy_vec3(vector: Vec3) -> Vector3<f32> {
    Vector3::new(vector.x, vector.y, vector.z)
}

#[inline]
pub fn to_bevy_vec3(vector: Vector3<f32>) -> Vec3 {
    Vec3::new(vector.x, vector.y, vector.z)
}

#[inline]
pub fn global_to_local_position(vector: Vector3<i32>) -> (Vector3<i32>, Vector3<usize>) {
    // Locate block
    let inner_loc = Vector3::new(
        ((vector.x as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.y as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.z as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
    );

    // Locate chunk
    let chunk_loc = Vector3::new(
        (vector.x as f32 / CHUNK_SIZE as f32).floor() as i32,
        (vector.y as f32 / CHUNK_SIZE as f32).floor() as i32,
        (vector.z as f32 / CHUNK_SIZE as f32).floor() as i32,
    );

    (chunk_loc, inner_loc)
}

pub enum TextureSubdivisionMethod {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Left,
    Right,
    Bottom,
    Full,
}

// pub struct AtlasIndex {
//     pub lookup: TextureAtlasIndex,
// }
//
// impl AtlasIndex {
//     pub fn new_lookup(name: &str) -> AtlasIndex {
//         let texture = ATLAS_LOOKUPS.get().unwrap().get(name);
//
//         let lookup = match texture {
//             Some(tex) => tex,
//             // Lookup error texture instead
//             None => {
//                 log_warn!("No texture index for {}", name);
//                 ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap()
//             }
//         };
//
//         AtlasIndex { lookup: *lookup }
//     }
//
//     pub fn get_subdivision(&self, subdivision: TextureSubdivisionMethod) -> TextureAtlasIndex {
//         match subdivision {
//             TextureSubdivisionMethod::TopLeft => {
//                 let width = self.lookup.width() / 2.0;
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup
//                     .local_offset(None, Some(-width), None, Some(-height))
//             }
//             TextureSubdivisionMethod::TopRight => {
//                 let width = self.lookup.width() / 2.0;
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup
//                     .local_offset(Some(width), None, None, Some(-height))
//             }
//             TextureSubdivisionMethod::BottomLeft => {
//                 let width = self.lookup.width() / 2.0;
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup
//                     .local_offset(None, Some(-width), Some(height), None)
//             }
//             TextureSubdivisionMethod::BottomRight => {
//                 let width = self.lookup.width() / 2.0;
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup
//                     .local_offset(Some(width), None, Some(height), None)
//             }
//             TextureSubdivisionMethod::Top => {
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup.local_offset(None, None, None, Some(-height))
//             }
//             TextureSubdivisionMethod::Left => {
//                 let width = self.lookup.width() / 2.0;
//
//                 self.lookup.local_offset(None, Some(-width), None, None)
//             }
//             TextureSubdivisionMethod::Right => {
//                 let width = self.lookup.width() / 2.0;
//
//                 self.lookup.local_offset(Some(width), None, None, None)
//             }
//             TextureSubdivisionMethod::Bottom => {
//                 let height = self.lookup.height() / 2.0;
//
//                 self.lookup.local_offset(None, None, Some(height), None)
//             }
//             TextureSubdivisionMethod::Full => self.lookup.clone(),
//         }
//     }
// }
