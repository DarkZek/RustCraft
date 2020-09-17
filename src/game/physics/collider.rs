use nalgebra::{Point3, Vector3};

#[derive(Debug)]
pub struct BoxCollider {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub center: Point3<f32>,
}

#[derive(Debug)]
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

    // Checks if another collider collides with this object
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

        let mut collision_side = CollisionSide::zero();
        let min_penetration = 0.1;

        let mut any_matched = false;

        /*

        https://github.com/ddevault/TrueCraft/blob/master/TrueCraft.Core/Physics/PhysicsEngine.cs

         */

        /*

        Ok Zekky, what you also need to do is you see that for loop down there? You gotta somehow get that working online, probably having to remove half
        of the surrounding code cause it has no idea what a `BoxCollider` is, ew. Then you take a yander at the machine code and idk how to tell if its
        simd or not, you can tell by if the compiled bit has longer instruction names that start with v.
        https://godbolt.org/
        https://software.intel.com/sites/landingpage/IntrinsicsGuide/

         */

        /*

        Ok zekky, move physics to own folder along like render/

         */

        for (matched, point) in points.iter_mut() {
            // Check if the point collided roughly
            if (point.x >= self.p1.x && point.x <= self.p2.x)
                && (point.y >= self.p1.y && point.y <= self.p2.y)
                && (point.z >= self.p1.z && point.z <= self.p2.z)
            {
                *matched = true;
                any_matched = true;

                // Always assume floor until we get side base collision done
                collision_side.bottom = true;

                if point.x >= self.p1.x
                    && point.x <= self.p2.x
                    && self.p2.y + min_penetration < point.y
                {
                    collision_side.front = true;
                    println!("Front");
                }
            }
        }

        if !any_matched {
            return None;
        }

        // Check for collision on top
        let _difference = Vector3::new(
            self.center.x - collider.center.x,
            self.center.y - collider.center.y,
            self.center.z - collider.center.z,
        );

        Some(collision_side)
    }
}

impl CollisionSide {
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

    pub fn invert(&mut self) {
        *self = CollisionSide {
            top: self.bottom,
            bottom: self.top,
            front: self.back,
            back: self.front,
            left: self.right,
            right: self.left,
        }
    }
}
