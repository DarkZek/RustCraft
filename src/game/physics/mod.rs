use cgmath::Vector3;

#[allow(dead_code)]
pub struct Hitbox {
    start: [f32; 2],
    end: [f32; 2]
}

pub trait PhysicsObject {
    fn get_hitbox(&self) -> Hitbox;

    fn get_velocity(&self) -> Vector3<f32>;

    fn set_velocity(&mut self, velocity: Vector3<f32>);

    fn translate(&mut self, direction: Vector3<f32>);
}

const DRAG: f32 = 0.2;

const VELOCITY_THRESHOLD: f32 = 0.01;

pub fn process_physics(objects: &mut Vec<Box<dyn PhysicsObject>>, delta_time: f64) {
    for object in objects {
        let mut velocity = object.get_velocity();
        velocity /= DRAG * delta_time as f32;
        object.set_velocity(velocity);

        // Dont do anything to non moving objects
        if velocity == [0.0, 0.0, 0.0].into() {
            continue;
        }

        // Apply threshold
        if velocity.x < VELOCITY_THRESHOLD { velocity.x = 0.0; }
        if velocity.y < VELOCITY_THRESHOLD { velocity.y = 0.0; }
        if velocity.z < VELOCITY_THRESHOLD { velocity.z = 0.0; }

        let movement = velocity * delta_time as f32;
        object.translate(movement);
    }
}