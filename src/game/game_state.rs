use crate::render::RenderState;
use crate::client::events::GameChanges;
use crate::entity::player::Player;
use std::f32::consts::PI;

pub struct GameState {
    player: Player
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: Player::new()
        }
    }

    pub fn frame(&mut self, render: &mut RenderState, events: &GameChanges, delta_time: f64) {

        let mut encoder = render.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        if events.look != [0.0, 0.0] {
            // They changed look
            let player = &mut self.player;
            let x_look_speed = 0.05;
            let y_look_speed = 0.05;

            // Update Horizontal Rotation
            player.rot[0] -= events.look[0] as f32 * x_look_speed * delta_time as f32;
            player.rot[0] %= (std::f32::consts::PI * 2.0);
            if player.rot[0] < 0.0 { player.rot[0] += std::f32::consts::PI * 2.0; }

            // Handle Vertical Rotation
            player.rot[1] = (player.rot[1] + (events.look[1] as f32 * y_look_speed * delta_time as f32)).clamp(0.01, std::f32::consts::PI - 0.01);

            render.camera.yaw = player.rot[0];
            render.camera.pitch = player.rot[1] - (PI / 2.0);
        }

        if events.movement != [0, 0] {
            self.player.move_forwards(&events.movement, delta_time.clone());

            // Update camera with change (assumes first person for now)
            render.camera.move_first_person(&self.player.pos);
        }

        if events.jump {
            self.player.pos[1] += 1.0;
            render.camera.move_first_person(&self.player.pos);
        }

        if events.sneak {
            self.player.pos[1] -= 1.0;
            render.camera.move_first_person(&self.player.pos);
        }

        render.uniforms.update_view_proj(&render.camera);

        let uniform_buffer = render.device
            .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[render.uniforms]);

        encoder.copy_buffer_to_buffer(&uniform_buffer, 0x0, &render.uniform_buffer, 0x0, std::mem::size_of_val(&render.uniforms) as wgpu::BufferAddress);

        render.queue.submit(&[encoder.finish()]);
    }
}