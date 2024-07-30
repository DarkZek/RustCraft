use std::time::Instant;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use nalgebra::Vector3;
use crate::game::events::DestroyBlockEvent;
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position};
use crate::systems::camera::freecam::Freecam;
use crate::systems::camera::MainCamera;

#[derive(Default)]
pub struct MouseInteractionLocals {
    left_clicking_started: Option<Instant>,
    left_clicking_target: Option<Vector3<i32>>
}

fn stop_clicking(locals: &mut MouseInteractionLocals) {
    if locals.left_clicking_target.is_some() {
        locals.left_clicking_started = None;
        locals.left_clicking_target = None;
    }
}

pub fn mouse_interaction_destroy(
    freecam: Res<Freecam>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<&Transform, With<MainCamera>>,
    mut destroy_block_event: EventWriter<DestroyBlockEvent>,
    mut chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
    mut locals: Local<MouseInteractionLocals>
) {

    if freecam.enabled {
        return;
    }

    if !mouse_button_input.pressed(MouseButton::Left) {
        // If they were interacting, they're not anymore
        stop_clicking(&mut locals);
        return;
    }

    let camera_pos = camera.get_single().unwrap();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        15.0,
        &chunks,
        &blocks,
    );

    if cast.is_none() {
        stop_clicking(&mut locals);
        return;
    }

    let ray = cast.unwrap();

    // Locate chunk
    let (chunk_loc, inner_loc) = global_to_local_position(ray.block);

    // Try find chunk
    let Some(chunk) = chunks.chunks.get(&chunk_loc) else {
        stop_clicking(&mut locals);
        return
    };

    if let Some(target) = &mut locals.left_clicking_target {
        if *target != ray.block {
            // Switched target block!
            locals.left_clicking_started = Some(Instant::now());
            locals.left_clicking_target = Some(ray.block);
        }
    } else {
        locals.left_clicking_started = Some(Instant::now());
        locals.left_clicking_target = Some(ray.block);
    }

    let block_id = chunk.world[inner_loc.x][inner_loc.y][inner_loc.z];

    if locals.left_clicking_started.unwrap().elapsed().as_millis() > 1000 {
        destroy_block_event.send(DestroyBlockEvent {
            player_triggered: true,
            position: ray.block,
            block_id,
        });
    }
}
