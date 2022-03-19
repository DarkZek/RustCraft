use crate::entity::player::{move_forwards, PlayerEntity};
use crate::game::game_state::GameState;
use crate::game::physics::{move_entity_dir, PhysicsObject};

use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::InputState;
use nalgebra::Vector3;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::Mul;

/// The base speed of the player
const BASE_MOVEMENT_SPEED: f32 = 0.2;

/// Multiplied by the movement speed when in the air
const AIR_MOVEMENT_RETENTION: f32 = 0.6;

/// Multiplied by the movement speed when sprinting
const SPRINT_MOVEMENT_MULTIPLIER: f32 = 2.1;

/// Amount of movement to apply as velocity
const MOVEMENT_VELOCITY_RATIO: f32 = 0.1;

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        Read<'a, InputState>,
        Read<'a, ActionSheet>,
        Write<'a, GameState>,
        ReadStorage<'a, PlayerEntity>,
        WriteStorage<'a, PhysicsObject>,
        ReadStorage<'a, ChunkData>,
        Read<'a, ChunkEntityLookup>,
    );

    fn run(
        &mut self,
        (
            events,
            actionsheet,
            game_state,
            player_entity,
            mut player_physics,
            chunks,
            chunk_entity_lookup,
        ): Self::SystemData,
    ) {
        let (_, entity) = (&player_entity, &mut player_physics).join().last().unwrap();

        let mut movement_modifier = BASE_MOVEMENT_SPEED;

        if entity.touching_ground {
            if actionsheet.get_sprinting() {
                movement_modifier *= SPRINT_MOVEMENT_MULTIPLIER;
            }
        } else {
            // Slow movement when in air
            movement_modifier *= AIR_MOVEMENT_RETENTION;
        }

        if events.movement != [0, 0] {
            // Update camera with change (assumes first person for now)
            let mut movement: Vector3<f32> =
                move_forwards(&events.movement, game_state.player.rot[0]).into();

            movement = movement.mul(movement_modifier);

            // Check collisions on three axis separately to allow for wall sliding

            // Check collisions
            let (final_movement, _collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(movement.x, 0.0, 0.0),
                entity.new_position,
            );

            entity.new_position += final_movement;

            // Check collisions
            let (final_movement, _collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, 0.0, movement.z),
                entity.new_position,
            );

            entity.new_position += final_movement;

            // Check collisions
            let (final_movement, _collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, movement.y, 0.0),
                entity.new_position,
            );

            entity.new_position += final_movement;
            // Add only a 1/10 of the force to the velocity so it still feels like we have force, but without the effects of stacking velocity
            entity.velocity += movement.mul(movement_modifier * MOVEMENT_VELOCITY_RATIO);
        }
    }
}
