use nalgebra::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct BoxCollider {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl BoxCollider {
    pub fn blank() -> BoxCollider {
        BoxCollider {
            min: Point3::new(0.0, 0.0, 0.0),
            max: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn shift(&self, position: Vector3<f32>) -> Self {
        BoxCollider {
            min: self.min + position,
            max: self.max + position,
        }
    }
    pub fn delta(&self, box_collider: BoxCollider) -> Vector3<f32> {
        box_collider.min - self.min
    }

    pub fn collides(&self, other: &BoxCollider) -> bool {
        !(other.min.x >= self.max.x
            || other.max.x <= self.min.x
            || other.min.y >= self.max.y
            || other.max.y <= self.min.y
            || other.min.z >= self.max.z
            || other.max.z <= self.min.z)
    }

    pub fn move_out_of(mut self, other: Self, dir: Vector3<f32>) -> Self {
        if dir.x != 0.0 {
            if dir.x > 0.0 {
                let ox = self.max.x;
                self.max.x = other.min.x - 0.0001;
                self.min.x += self.max.x - ox;
            } else {
                let ox = self.min.x;
                self.min.x = other.max.x + 0.0001;
                self.max.x += self.min.x - ox;
            }
        }
        if dir.y != 0.0 {
            if dir.y > 0.0 {
                let oy = self.max.y;
                self.max.y = other.min.y - 0.0001;
                self.min.y += self.max.y - oy;
            } else {
                let oy = self.min.y;
                self.min.y = other.max.y + 0.0001;
                self.max.y += self.min.y - oy;
            }
        }
        if dir.z != 0.0 {
            if dir.z > 0.0 {
                let oz = self.max.z;
                self.max.z = other.min.z - 0.0001;
                self.min.z += self.max.z - oz;
            } else {
                let oz = self.min.z;
                self.min.z = other.max.z + 0.0001;
                self.max.z += self.min.z - oz;
            }
        }
        self
    }
}
