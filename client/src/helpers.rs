use crate::systems::chunk::data::LightingColor;
use bevy::prelude::Vec3;
use nalgebra::{Point3, Vector3};
use rc_networking::constants::CHUNK_SIZE;
use std::ops::Add;

/// Formats a u32 with American comma placement.
///
/// # Example
/// ```rust
/// use rc_client::helpers::format_u32;
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
    /// Linearly interpolates between `self` and `b` by `t`
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

/// Linearly interpolates between two colours by `t` within the range 0-1
pub fn lerp_color(c1: LightingColor, c2: LightingColor, t: f32) -> LightingColor {
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

/// Calculate the distance between two `Point3` points
pub fn distance(p1: &Point3<usize>, p2: &Point3<usize>) -> u32 {
    ((p1.x as isize - p2.x as isize).abs()
        + (p1.y as isize - p2.y as isize).abs()
        + (p1.z as isize - p2.z as isize).abs())
    .abs() as u32
}

pub trait Clamp {
    /// Clamp `self` value between `min` and `max`
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

/// Converts from global block position referencing any block in the entire world, to local array indexing position of 0-CHUNK_SIZE and a position of the chunk in the world
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

/// Converts from global block position referencing any block in the entire world, to local array indexing position of 0-CHUNK_SIZE and a position of the chunk in the world
#[inline]
pub fn global_f32_to_local_position(vector: Vector3<f32>) -> (Vector3<i32>, Vector3<usize>) {
    // Locate block
    let inner_loc = Vector3::new(
        ((vector.x as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.y as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.z as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
    );

    // Locate chunk
    let chunk_loc = Vector3::new(
        (vector.x / CHUNK_SIZE as f32).floor() as i32,
        (vector.y / CHUNK_SIZE as f32).floor() as i32,
        (vector.z / CHUNK_SIZE as f32).floor() as i32,
    );

    (chunk_loc, inner_loc)
}

#[inline]
/// Returns true when a position moved by a direction is still within the 0-15 chunk boundaries
pub fn check_chunk_boundaries(pos: Vector3<usize>, dir: Vector3<i32>) -> bool {
    match (dir.x, dir.y, dir.z) {
        (1, 0, 0) => pos.x < CHUNK_SIZE - 1,
        (-1, 0, 0) => pos.x > 0,
        (0, 1, 0) => pos.y < CHUNK_SIZE - 1,
        (0, -1, 0) => pos.y > 0,
        (0, 0, 1) => pos.z < CHUNK_SIZE - 1,
        (0, 0, -1) => pos.z > 0,

        _ => panic!("Invalid direction"),
    }
}
