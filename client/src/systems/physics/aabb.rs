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

    pub fn ray_collides(
        &self,
        offset: Vector3<f32>,
        starting_position: Vector3<f32>,
        direction: Vector3<f32>,
    ) -> (bool, f32) {
        let lb = self.bottom_left + offset;
        let rt = lb + self.size;

        // r.dir is unit direction vector of ray
        let dirfrac = Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        // lb is the corner of AABB with minimal coordinates - left bottom, rt is maximal corner
        // r.org is origin of ray
        let t1 = (lb.x - starting_position.x) * dirfrac.x;
        let t2 = (rt.x - starting_position.x) * dirfrac.x;
        let t3 = (lb.y - starting_position.y) * dirfrac.y;
        let t4 = (rt.y - starting_position.y) * dirfrac.y;
        let t5 = (lb.z - starting_position.z) * dirfrac.z;
        let t6 = (rt.z - starting_position.z) * dirfrac.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0.0 {
            (false, tmax)
        } else if tmin > tmax {
            // if tmin > tmax, ray doesn't intersect AABB
            (false, tmax)
        } else {
            (true, tmin)
        }
    }
}
