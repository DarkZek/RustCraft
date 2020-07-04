use crate::entity::player::Player;
use crate::render::RenderState;
use std::f32::consts::PI;
use crate::services::input_service::input::GameChanges;
use specs::{System, Write, Read};
use crate::game::systems::DeltaTime;
use crate::render::camera::Camera;

/// Stores the current state of the game. Currently this is mostly just looking after player movement.
pub struct GameState {
    player: Player,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: Player::new()
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        unimplemented!()
    }
}

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {

    type SystemData = (Write<'a, RenderState>,
                        Read<'a, GameChanges>,
                        Read<'a, DeltaTime>,
                        Write<'a, Camera>,
                        Write<'a, GameState>);

    fn run(&mut self, (mut render, events, delta_time, mut camera, mut game_state): Self::SystemData) {

        let mut encoder = render
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if events.look != [0.0, 0.0] {
            // They changed look
            let player = &mut game_state.player;
            let x_look_speed = 0.005;
            let y_look_speed = 0.005;

            // Update Horizontal Rotation
            player.rot[0] -= events.look[0] as f32 * x_look_speed;
            player.rot[0] %= std::f32::consts::PI * 2.0;
            if player.rot[0] < 0.0 {
                player.rot[0] += std::f32::consts::PI * 2.0;
            }

            // Handle Vertical Rotation
            player.rot[1] = (player.rot[1] + (events.look[1] as f32 * y_look_speed))
                .clamp(0.01, std::f32::consts::PI - 0.01);

            camera.yaw = player.rot[0];
            camera.pitch = player.rot[1] - (PI / 2.0);

            // let mut services = render.services.take().unwrap();
            // services.chunk.update_frustum_culling(&render.camera);
            // render.services = Some(services);
        }

        if events.movement != [0, 0] {
            game_state.player
                .move_forwards(&events.movement);

            // Update camera with change (assumes first person for now)
            camera.move_first_person(&game_state.player.pos);
        }

        if events.jump {
            game_state.player.pos[1] += 1.0;
            camera.move_first_person(&game_state.player.pos);
        }

        if events.sneak {
            game_state.player.pos[1] -= 1.0;
            camera.move_first_person(&game_state.player.pos);
        }

        render.uniforms.update_view_proj(&camera);

        let uniform_buffer = render
            .device
            .create_buffer_with_data(bytemuck::cast_slice(&render.uniforms.view_proj), wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC);

        encoder.copy_buffer_to_buffer(
            &uniform_buffer,
            0x0,
            &render.uniform_buffer,
            0x0,
            std::mem::size_of_val(&render.uniforms) as wgpu::BufferAddress,
        );

        render.queue.submit(&[encoder.finish()]);
    }

}
