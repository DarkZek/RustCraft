use crate::block::blocks::BlockType;
use crate::game::physics::collider::BoxCollider;
use crate::world::WorldChunks;
use nalgebra::Vector3;

impl WorldChunks<'a> {
    pub fn do_raycast(&self, cast: Raycast) -> Option<Vector3<i64>> {
        let query_blocks = self.get_raycast_candidates(&cast);

        let mut closest_position = None;
        let mut closest_distance = None;

        for (position, block) in query_blocks {
            for collider in block.get_collision_boxes() {
                let collider =
                    collider.shift(nalgebra::convert::<Vector3<i64>, Vector3<f32>>(position));

                let (hit, distance) = cast.single_raycast(collider);

                if hit {
                    if closest_distance.is_none() || closest_distance.unwrap() > distance {
                        closest_position = Some(position);
                        closest_distance = Some(distance);
                    }
                }
            }
        }

        closest_position
    }

    fn get_raycast_candidates(&self, cast: &Raycast) -> Vec<(Vector3<i64>, &'a BlockType)> {
        let mut positions = Vec::new();

        // Move along line
        for i in 0..cast.max_length {
            let pos = cast.origin + (cast.direction * i as f32);

            let mut query_positions = Vec::with_capacity(2 * 2 * 2);
            for x in 0..=1 {
                for _y in 0..=1 {
                    for _z in 0..=1 {
                        let x = if x == 0 {
                            pos.x.floor() as i64
                        } else {
                            pos.x.ceil() as i64
                        };
                        let y = if x == 0 {
                            pos.y.floor() as i64
                        } else {
                            pos.y.ceil() as i64
                        };
                        let z = if x == 0 {
                            pos.z.floor() as i64
                        } else {
                            pos.z.ceil() as i64
                        };

                        query_positions.push(Vector3::new(x, y, z))
                    }
                }
            }

            for position in query_positions {
                // Get every possible block
                if let Some(block) = self.get_block(position) {
                    positions.push((position, block));
                }
            }
        }

        positions
    }
}

pub struct Raycast {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
    inverted_direction: Vector3<f32>,
    max_length: usize,
}

impl Raycast {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, max_length: usize) -> Raycast {
        // Invert direction
        let inverted_direction =
            Vector3::new(1.0 / direction[0], 1.0 / direction[1], 1.0 / direction[2]);

        Raycast {
            origin,
            direction,
            inverted_direction,
            max_length,
        }
    }

    /// Gets the distance of the box using a raycast
    /// Based on https://tavianatoself.com/2011/ray_box.html
    pub fn single_raycast(&self, model: BoxCollider) -> (bool, f32) {
        let mut t1 = (model.min[0] - self.origin.x) * self.inverted_direction.x;
        let mut t2 = (model.max[0] - self.origin.x) * self.inverted_direction.x;

        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);

        t1 = (model.min[1] - self.origin.y) * self.inverted_direction.y;
        t2 = (model.max[1] - self.origin.y) * self.inverted_direction.y;

        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        t1 = (model.min[2] - self.origin.z) * self.inverted_direction.z;
        t2 = (model.max[2] - self.origin.z) * self.inverted_direction.z;

        tmin = t1.min(t2).max(tmin);
        tmax = t1.max(t2).min(tmax);

        if tmax >= tmin.max(0.0) {
            return (true, tmin);
        }

        return (false, 0.0);
    }
}
