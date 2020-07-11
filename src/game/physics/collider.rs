use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct BoxCollider {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub center: Point3<f32>,
}

//TODO: Convert this to a byte flag down the line
pub struct CollisionSide {
    pub top: bool,
    pub bottom: bool,
    pub front: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

impl BoxCollider {
    pub fn blank() -> BoxCollider {
        BoxCollider {
            p1: Point3::new(0.0, 0.0, 0.0),
            p2: Point3::new(0.0, 0.0, 0.0),
            center: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn check_collision(&self, collider: &BoxCollider) -> Option<CollisionSide> {
        let mut points = [
            (
                false,
                Point3::new(collider.p1.x, collider.p1.y, collider.p1.z),
            ),
            (
                false,
                Point3::new(collider.p2.x, collider.p1.y, collider.p1.z),
            ),
            (
                false,
                Point3::new(collider.p1.x, collider.p1.y, collider.p2.z),
            ),
            (
                false,
                Point3::new(collider.p2.x, collider.p1.y, collider.p2.z),
            ),
            (
                false,
                Point3::new(collider.p1.x, collider.p2.y, collider.p1.z),
            ),
            (
                false,
                Point3::new(collider.p2.x, collider.p2.y, collider.p1.z),
            ),
            (
                false,
                Point3::new(collider.p1.x, collider.p2.y, collider.p2.z),
            ),
            (
                false,
                Point3::new(collider.p2.x, collider.p2.y, collider.p2.z),
            ),
        ];

        let mut any_matched = false;

        for (matched, point) in points.iter_mut() {
            if (point.x >= self.p1.x && point.x <= self.p2.x)
                && (point.y >= self.p1.y && point.y <= self.p2.y)
                && (point.z >= self.p1.z && point.z <= self.p2.z)
            {
                *matched = true;
                any_matched = true;
            }
        }

        if !any_matched {
            return None;
        }

        // Check for collision on top
        let difference = Vector3::new(
            self.center.x - collider.center.x,
            self.center.y - collider.center.y,
            self.center.z - collider.center.z,
        );

        Some(CollisionSide::from_vector3(difference.normalize()))
    }
}

impl CollisionSide {
    pub fn from_vector3(vector: Vector3<f32>) -> CollisionSide {
        let mut collision = CollisionSide {
            top: false,
            bottom: false,
            front: false,
            back: false,
            left: false,
            right: false,
        };

        if vector.y > 0.5 {
            collision.top = true;
        } else if vector.y < 0.5 {
            collision.bottom = true;
        }

        if vector.z > 0.5 {
            collision.right = true;
        } else if vector.z < 0.5 {
            collision.left = true;
        }

        if vector.x > 0.5 {
            collision.front = true;
        } else if vector.x < 0.5 {
            collision.back = true;
        }

        collision
    }

    pub fn zero() -> CollisionSide {
        CollisionSide {
            top: false,
            bottom: false,
            front: false,
            back: false,
            left: false,
            right: false,
        }
    }

    pub fn combine(&mut self, input: &CollisionSide) {
        if input.top {
            self.top = true;
        }
        if input.bottom {
            self.bottom = true;
        }
        if input.left {
            self.left = true;
        }
        if input.right {
            self.right = true;
        }
        if input.front {
            self.front = true;
        }
        if input.back {
            self.back = true;
        }
    }
}
