use crate::entity::player::{move_forwards, Player, PlayerEntity};
use crate::game::physics::PhysicsObject;
use crate::helpers::Clamp;
use crate::render::camera::Camera;
use crate::render::RenderState;
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::GameChanges;
use specs::{Builder, Join, Read, ReadStorage, System, World, WorldExt, Write, WriteStorage};
use std::f32::consts::PI;

/// Stores the current state of the game. Currently this is mostly just looking after player movement.
pub struct GameState {
    player: Player,
}

impl GameState {
    pub fn new(universe: &mut World) -> GameState {
        universe
            .create_entity()
            .with(PhysicsObject::new())
            .with(PlayerEntity)
            .build();

        GameState {
            player: Player::new(),
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
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, GameChanges>,
        Write<'a, Camera>,
        Write<'a, GameState>,
        ReadStorage<'a, PlayerEntity>,
        WriteStorage<'a, PhysicsObject>,
        Write<'a, ActionSheet>,
    );

    fn run(
        &mut self,
        (
            mut render,
            events,
            mut camera,
            mut game_state,
            player_entity,
            mut player_physics,
            mut actionsheet,
        ): Self::SystemData,
    ) {
        let mut encoder = render
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if events.look != [0.0, 0.0] {
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
                .clamp_val(0.01, std::f32::consts::PI - 0.01);

            camera.yaw = player.rot[0];
            camera.pitch = player.rot[1] - (PI / 2.0);
        }

        if events.movement != [0, 0] {
            //TODO: Try make a macro out of this, I tried once but it kept saying it could find the macro :(
            let (_, player_physics) = (&player_entity, &mut player_physics).join().last().unwrap();

            // Update camera with change (assumes first person for now)
            player_physics.velocity =
                move_forwards(&events.movement, game_state.player.rot[0]).into();
        }

        if events.jump {
            actionsheet.set_jump();
        }

        if actionsheet.get_jump() {
            let (_, player_physics) = (&player_entity, &mut player_physics).join().last().unwrap();
            if player_physics.touching_ground {
                player_physics.velocity.y += 0.42;
            }
        }

        render.uniforms.update_view_proj(&mut camera);

        let uniform_buffer = render.device.create_buffer_with_data(
            bytemuck::cast_slice(&render.uniforms.view_proj),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &uniform_buffer,
            0x0,
            &render.uniform_buffer,
            0x0,
            std::mem::size_of_val(&render.uniforms) as wgpu::BufferAddress,
        );

        render.queue.submit(Some(encoder.finish()));
    }
}
