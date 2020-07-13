use std::ops::{Add};
use crate::services::chunk_service::chunk::Color;
use nalgebra::Point3;

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
    fn lerp(self, b: Self, t: f32) -> f32;
}

impl Lerp for f32 {
    fn lerp(self, b: Self, t: f32) -> f32 {
        ((b - self) * t) + self
    }
}

impl Lerp for u8 {
    fn lerp(self, b: Self, t: f32) -> f32 {
        ((b as f32 - self as f32) * t as f32) + self as f32
    }
}

pub fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    [
        c1[0].lerp(c2[0], t),
        c1[1].lerp(c2[1], t),
        c1[2].lerp(c2[2], t),
        c1[3].lerp(c2[3], t),
    ]
}

pub fn distance(p1: &Point3<usize>, p2: &Point3<usize>) -> u32 {
    ((p1.x as isize - p2.x as isize).abs() +
        (p1.y as isize - p2.y as isize).abs() +
        (p1.z as isize - p2.z as isize).abs()).abs() as u32
}