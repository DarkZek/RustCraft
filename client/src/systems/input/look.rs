use crate::game::player::Player;
use crate::systems::input::InputSystem;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.00022;

pub fn update_input_look(
    service: Res<InputSystem>,
    mut mouse: EventReader<MouseMotion>,
    mut player: Query<(&mut Transform, &mut Player)>,
    windows: Res<Windows>,
) {
    if !service.captured {
        return;
    }

    let window = windows.get_primary().unwrap();

    let window_scale = window.height().min(window.width());

    let (mut transform, mut player) = player.single_mut();

    for motion in mouse.iter() {
        player.pitch -= (MOUSE_SENSITIVITY * motion.delta.y * window_scale).to_radians();
        player.yaw -= (MOUSE_SENSITIVITY * motion.delta.x * window_scale).to_radians();
        
        // Prevent the player from looking too far up or down
        player.pitch = player.pitch.min(std::f32::consts::FRAC_PI_2);
        player.pitch = player.pitch.max(-std::f32::consts::FRAC_PI_2);
        
        transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw)
            * Quat::from_axis_angle(Vec3::X, player.pitch);
    }
}
