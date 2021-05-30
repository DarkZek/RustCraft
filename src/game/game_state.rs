use crate::entity::player::{move_forwards, Player, PlayerEntity};
use crate::game::physics::PhysicsObject;
use crate::helpers::Clamp;
use crate::render::camera::Camera;
use crate::render::RenderState;
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::{GameChanges, InputChange};
use nalgebra::Vector3;
use specs::{Builder, Join, Read, ReadStorage, System, World, WorldExt, Write, WriteStorage};
use std::f32::consts::PI;
use std::ops::Mul;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

/// Stores the current state of the game. Currently this is mostly just looking after player movement.
pub struct GameState {
    pub state: ProgramState,
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
            state: ProgramState::INIT,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(PartialEq)]
pub enum ProgramState {
    Init,
    Menu,
    Loading,
    InGame,
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
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Player movement command encoder"),
            });

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

        let mut movement_modifier = 0.25;

        if actionsheet.get_sprinting() {
            movement_modifier *= 2.3;
        }

        if events.movement != [0, 0] {
            //TODO: Try make a macro out of this, I tried once but it kept saying it could find the macro :(
            let (_, player_physics) = (&player_entity, &mut player_physics).join().last().unwrap();

            // Update camera with change (assumes first person for now)
            let movement: Vector3<f32> =
                move_forwards(&events.movement, game_state.player.rot[0]).into();

            player_physics.position += movement.mul(movement_modifier);
            // Add only a 1/10 of the force to the velocity so it still feels like we have force, but without the effects of stacking velocity
            player_physics.velocity += movement.mul(movement_modifier / 10.0);
        }

        if events.jump {
            actionsheet.set_jump();
        }

        if actionsheet.get_jump() {
            let (_, player_physics) = (&player_entity, &mut player_physics).join().last().unwrap();
            if player_physics.touching_ground {
                player_physics.velocity.y += 0.42 * 1.25;
            }
        }

        if events.ctrl != InputChange::None {
            if events.ctrl == InputChange::Released {
                actionsheet.set_sprinting(false)
            } else {
                actionsheet.set_sprinting(true)
            }
        }

        render.uniforms.update_view_proj(&mut camera);

        let uniform_buffer = render.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("View Projection Buffer"),
            contents: &bytemuck::cast_slice(&render.uniforms.view_proj),
            usage: wgpu::BufferUsage::UNIFORM
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        });

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
