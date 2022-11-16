use bevy::prelude::{ResMut, Vec3};
use bevy_prototype_debug_lines::DebugLines;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Aabb {
    bottom_left: Vector3<f32>,
    size: Vector3<f32>,
}

impl Aabb {
    pub fn new(bottom_left: Vector3<f32>, size: Vector3<f32>) -> Aabb {
        Aabb { bottom_left, size }
    }

    pub fn draw_lines(boxes: &Vec<Aabb>, position: Vector3<f32>, lines: &mut ResMut<DebugLines>) {
        for val in boxes {
            let base = val.bottom_left + position;

            // Bottom ring
            lines.line(
                Vec3::new(base.x, base.y, base.z),
                Vec3::new(base.x + val.size.x, base.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x, base.y, base.z),
                Vec3::new(base.x, base.y, base.z + val.size.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x + val.size.x, base.y, base.z + val.size.z),
                Vec3::new(base.x + val.size.x, base.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x + val.size.x, base.y, base.z + val.size.z),
                Vec3::new(base.x, base.y, base.z + val.size.z),
                0.0,
            );

            // Top ring
            lines.line(
                Vec3::new(base.x, base.y + val.size.y, base.z),
                Vec3::new(base.x + val.size.x, base.y + val.size.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x, base.y + val.size.y, base.z),
                Vec3::new(base.x, base.y + val.size.y, base.z + val.size.z),
                0.0,
            );
            lines.line(
                Vec3::new(
                    base.x + val.size.x,
                    base.y + val.size.y,
                    base.z + val.size.z,
                ),
                Vec3::new(base.x + val.size.x, base.y + val.size.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(
                    base.x + val.size.x,
                    base.y + val.size.y,
                    base.z + val.size.z,
                ),
                Vec3::new(base.x, base.y + val.size.y, base.z + val.size.z),
                0.0,
            );

            // Vertical ring
            lines.line(
                Vec3::new(base.x, base.y, base.z),
                Vec3::new(base.x, base.y + val.size.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x, base.y, base.z + val.size.z),
                Vec3::new(base.x, base.y + val.size.y, base.z + val.size.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x + val.size.x, base.y, base.z),
                Vec3::new(base.x + val.size.x, base.y + val.size.y, base.z),
                0.0,
            );
            lines.line(
                Vec3::new(base.x + val.size.x, base.y, base.z + val.size.z),
                Vec3::new(
                    base.x + val.size.x,
                    base.y + val.size.y,
                    base.z + val.size.z,
                ),
                0.0,
            );
        }
    }
}
