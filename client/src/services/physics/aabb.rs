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
            val.draw(position, lines, 0.0);
        }
    }

    pub fn draw(&self, position: Vector3<f32>, lines: &mut ResMut<DebugLines>, duration: f32) {
        let base = self.bottom_left + position;

        // Bottom ring
        lines.line(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x + self.size.x, base.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x, base.y, base.z + self.size.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(base.x + self.size.x, base.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(base.x, base.y, base.z + self.size.z),
            duration,
        );

        // Top ring
        lines.line(
            Vec3::new(base.x, base.y + self.size.y, base.z),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x, base.y + self.size.y, base.z),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
        );
        lines.line(
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
        );

        // Vertical ring
        lines.line(
            Vec3::new(base.x, base.y, base.z),
            Vec3::new(base.x, base.y + self.size.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x, base.y, base.z + self.size.z),
            Vec3::new(base.x, base.y + self.size.y, base.z + self.size.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x + self.size.x, base.y, base.z),
            Vec3::new(base.x + self.size.x, base.y + self.size.y, base.z),
            duration,
        );
        lines.line(
            Vec3::new(base.x + self.size.x, base.y, base.z + self.size.z),
            Vec3::new(
                base.x + self.size.x,
                base.y + self.size.y,
                base.z + self.size.z,
            ),
            duration,
        );
    }
}
