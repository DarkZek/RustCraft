use crate::entity::player::{Player, PlayerEntity};
use crate::game::physics::PhysicsObject;
use crate::helpers::Clamp;
use crate::render::camera::Camera;
use crate::render::device::get_device;
use crate::render::pass::uniforms::RenderViewProjectionUniforms;
use crate::render::RenderState;
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::{InputChange, InputState};

use specs::{Builder, Join, Read, ReadStorage, System, World, WorldExt, Write, WriteStorage};
use std::f32::consts::PI;

/// Stores the current state of the game. Currently this is mostly just looking after player movement.
pub struct GameState {
    pub state: ProgramState,
    pub player: Player,
}

impl GameState {
    pub fn new(universe: &mut World) -> GameState {
        universe
            .create_entity()
            .with(PlayerEntity::create_physics_object())
            .with(PlayerEntity)
            .build();

        GameState {
            player: Player::new(),
            state: ProgramState::Init,
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

pub struct PlayerActionsSystem;

impl<'a> System<'a> for PlayerActionsSystem {
    type SystemData = (
        Write<'a, RenderState>,
        Read<'a, InputState>,
        Write<'a, Camera>,
        Write<'a, GameState>,
        ReadStorage<'a, PlayerEntity>,
        WriteStorage<'a, PhysicsObject>,
        Write<'a, ActionSheet>,
    );

    fn run(
        &mut self,
        (
            render,
            events,
            mut camera,
            mut game_state,
            player_entity,
            mut player_physics,
            mut actionsheet,
        ): Self::SystemData,
    ) {
        let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        if events.jump {
            actionsheet.set_jump();
        }

        if actionsheet.get_jump() {
            let (_, player_physics) = (&player_entity, &mut player_physics).join().last().unwrap();
            if player_physics.touching_ground {
                player_physics.velocity.y += 0.7;
            }
        }

        if events.ctrl != InputChange::None {
            if events.ctrl == InputChange::Released {
                actionsheet.set_sprinting(false)
            } else {
                actionsheet.set_sprinting(true)
            }
        }

        RenderViewProjectionUniforms::update_uniform_buffers(
            &mut camera,
            &mut encoder,
            &render.uniform_buffer,
        );

        render.queue.submit(Some(encoder.finish()));
    }
}
