use crate::chunk::{ChunkPosition, LightingColor, LocalBlockPosition};
use crate::CHUNK_SIZE;
use bevy::prelude::Vec3;
use nalgebra::{Point3, Vector3};
use std::ops::{Add, Range};

/// Formats a u32 with American comma placement.
///
/// # Example
/// ```rust
/// use rc_shared::helpers::format_u32;
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
    LightingColor {
        r: if c1.r < c2.r {
            c1.r.lerp(c2.r, t)
        } else {
            c2.r.lerp(c1.r, t)
        },
        g: if c1.g < c2.g {
            c1.g.lerp(c2.g, t)
        } else {
            c2.g.lerp(c1.g, t)
        },
        b: if c1.b < c2.b {
            c1.b.lerp(c2.b, t)
        } else {
            c2.b.lerp(c1.b, t)
        },
        strength: if c1.strength < c2.strength {
            c1.strength.lerp(c2.strength, t)
        } else {
            c2.strength.lerp(c1.strength, t)
        },
        skylight: if c1.skylight < c2.skylight {
            c1.skylight.lerp(c2.skylight, t)
        } else {
            c2.skylight.lerp(c1.skylight, t)
        },
    }
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
pub fn global_to_local_position(vector: Vector3<i32>) -> (ChunkPosition, LocalBlockPosition) {
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

#[inline]
pub fn local_to_global_position(chunk_pos: Vector3<i32>, block_pos: Vector3<usize>) -> Vector3<i32> {
    // Locate block
    (chunk_pos * CHUNK_SIZE as i32) + block_pos.cast::<i32>()
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

/// Takes a `value` in range `input` and converts it to the range `output`
pub fn map(input: Range<f32>, output: Range<f32>, value: f32) -> f32 {
    let normalized = (value - input.start) / (input.end - input.start);
    (normalized * (output.end - output.start)) + output.start
}

#[test]
fn test_map() {
    assert_eq!(map(0.0..1.0, 0.0..10.0, 0.3), 3.0);
    assert_eq!(map(0.0..0.5, 0.0..10.0, 0.3), 6.0);
    assert_eq!(map(0.0..0.5, 10.0..0.0, 0.3), 4.0);
}
