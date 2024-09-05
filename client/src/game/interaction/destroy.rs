use web_time::Instant;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use nalgebra::Vector3;
use crate::game::events::DestroyBlockEvent;
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position, to_bevy_vec3};
use crate::game::interaction::MAX_INTERACTION_DISTANCE;
use crate::systems::camera::freecam::Freecam;
use crate::systems::camera::MainCamera;

#[derive(Default, Resource)]
pub struct MouseInteractionResource {
    pub left_clicking_started: Option<Instant>,
    pub left_clicking_target: Option<Vector3<i32>>,
    pub block_selection_entity: Option<Entity>
}

fn stop_clicking(
    locals: &mut MouseInteractionResource,
    transforms: &mut Query<(&mut Transform, &mut Visibility), Without<MainCamera>>,
) {
    if locals.left_clicking_target.is_some() {
        locals.left_clicking_started = None;
        locals.left_clicking_target = None;
        *transforms.get_mut(locals.block_selection_entity.unwrap()).unwrap().1 = Visibility::Hidden;
    }
}

pub fn mouse_interaction_destroy(
    freecam: Res<Freecam>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut transforms: ParamSet<(
        Query<(&mut Transform, &mut Visibility), Without<MainCamera>>,
        Query<&Transform, With<MainCamera>>,
    )>,
    mut destroy_block_event: EventWriter<DestroyBlockEvent>,
    chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
    mut locals: ResMut<MouseInteractionResource>
) {

    if freecam.enabled {
        return;
    }

    if !mouse_button_input.pressed(MouseButton::Left) {
        // If they were interacting, they're not anymore
        stop_clicking(&mut locals, &mut transforms.p0());
        return;
    }

    let camera_pos = transforms.p1().get_single().unwrap().clone();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        MAX_INTERACTION_DISTANCE,
        &chunks,
        &blocks,
    );

    if cast.is_none() {
        stop_clicking(&mut locals, &mut transforms.p0());
        return;
    }

    let ray = cast.unwrap();

    // Locate chunk
    let (chunk_loc, inner_loc) = global_to_local_position(ray.block);

    // Try find chunk
    let Some(chunk) = chunks.chunks.get(&chunk_loc) else {
        stop_clicking(&mut locals, &mut transforms.p0());
        return
    };

    let mut start_clicking = false;

    if let Some(target) = &mut locals.left_clicking_target {
        if *target != ray.block {
            // Switched target block!
            start_clicking = true;
        }
    } else {
        start_clicking = true;
    }

    if start_clicking {
        locals.left_clicking_started = Some(Instant::now());
        locals.left_clicking_target = Some(ray.block);
        let mut p0 = transforms.p0();
        let (mut transform, mut visibility) =
            p0.get_mut(locals.block_selection_entity.unwrap()).unwrap();
        *visibility.as_mut() = Visibility::Visible;
        transform.translation
            = to_bevy_vec3(ray.block.cast::<f32>()) + Vec3::new(0.5, 0.5, 0.5);
    }

    let block_id = chunk.world.get(inner_loc);

    if locals.left_clicking_started.unwrap().elapsed().as_millis() > 800 {
        destroy_block_event.send(DestroyBlockEvent {
            player_triggered: true,
            position: ray.block,
            block_id,
        });
    }
}
