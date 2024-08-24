use crate::systems::input::InputSystem;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::systems::camera::MainCamera;

const MOUSE_SENSITIVITY: f32 = 0.005;

pub fn update_input_look(
    service: Res<InputSystem>,
    mut mouse: EventReader<MouseMotion>,
    mut player: Query<&mut Transform, With<MainCamera>>,
) {
    if !service.captured {
        return;
    }

    let mut transform = player.single_mut();

    for motion in mouse.read() {
        // transform.rotate_local_axis(Dir3::new(Vec3::new(1.0, 0.0, 0.0)).unwrap(), MOUSE_SENSITIVITY * -motion.delta.y);
        // transform.rotation *= Quat::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), MOUSE_SENSITIVITY * -motion.delta.y);
        let (x, mut y, z) = transform.rotation.to_euler(EulerRot::YXZ);

        y += MOUSE_SENSITIVITY * -motion.delta.y;

        y = y.clamp(-1.5, 1.5);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, x, y, z);

        transform.rotate_axis(Dir3::new(Vec3::new(0.0, 1.0, 0.0)).unwrap(), MOUSE_SENSITIVITY * -motion.delta.x);
    }
}
