use crate::entity::player::{move_forwards, PlayerEntity};
use crate::game::game_state::GameState;
use crate::game::physics::{move_entity_dir, PhysicsObject};
use crate::render::camera::Camera;
use crate::render::RenderState;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::services::input_service::actions::ActionSheet;
use crate::services::input_service::input::InputState;
use nalgebra::Vector3;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::Mul;

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
            mut game_state,
            player_entity,
            mut player_physics,
            chunks,
            chunk_entity_lookup,
        ): Self::SystemData,
    ) {
        let (_, entity) = (&player_entity, &mut player_physics).join().last().unwrap();

        let mut movement_modifier = 0.12;

        if entity.touching_ground {
            if actionsheet.get_sprinting() {
                movement_modifier *= 2.1;
            }
        } else {
            // Slow movement when in air
            movement_modifier * 0.4;
        }

        if events.movement != [0, 0] {
            //TODO: Try make a macro out of this, I tried once but it kept saying it could find the macro :(

            // Update camera with change (assumes first person for now)
            let mut movement: Vector3<f32> =
                move_forwards(&events.movement, game_state.player.rot[0]).into();

            movement = movement.mul(movement_modifier);

            // Check collisions
            let (final_movement, collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(movement.x, 0.0, 0.0),
                entity.new_position,
            );

            entity.new_position += final_movement;

            // Check collisions
            let (final_movement, collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, 0.0, movement.z),
                entity.new_position,
            );

            entity.new_position += final_movement;

            // Check collisions
            let (final_movement, collision) = move_entity_dir(
                &entity.collider,
                &chunks,
                &chunk_entity_lookup,
                Vector3::new(0.0, movement.y, 0.0),
                entity.new_position,
            );

            entity.new_position += final_movement;
            // Add only a 1/10 of the force to the velocity so it still feels like we have force, but without the effects of stacking velocity
            entity.velocity += movement.mul(movement_modifier / 10.0);
        }
    }
}
