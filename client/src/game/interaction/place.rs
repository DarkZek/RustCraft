use web_time::Instant;
use crate::systems::chunk::ChunkSystem;
use crate::systems::physics::raycasts::do_raycast;
use bevy::prelude::*;
use nalgebra::Vector3;
use crate::game::inventory::Inventory;
use crate::systems::chunk::builder::{RerenderChunkRequest, RerenderChunkFlagContext};
use rc_shared::constants::UserId;
use rc_networking::protocol::Protocol;
use rc_networking::protocol::serverbound::place_block::PlaceBlock;
use rc_networking::types::SendPacket;
use rc_shared::aabb::Aabb;
use rc_shared::block::BlockStates;
use rc_shared::helpers::{from_bevy_vec3, global_to_local_position};
use crate::game::entity::GameObject;
use crate::game::interaction::MAX_INTERACTION_DISTANCE;
use crate::systems::camera::Freecam;
use crate::systems::camera::MainCamera;
use crate::systems::physics::PhysicsObject;

pub struct MouseInteractionLocals {
    left_clicking_started: Option<Instant>,
    left_clicking_target: Vector3<i32>
}

pub fn mouse_interaction_place(
    freecam: Res<Freecam>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<&Transform, With<MainCamera>>,
    mut rerender_chunk_event: EventWriter<RerenderChunkRequest>,
    mut chunks: ResMut<ChunkSystem>,
    blocks: Res<BlockStates>,
    mut inventory: ResMut<Inventory>,
    mut send_packet: EventWriter<SendPacket>,
    game_objects: Query<&PhysicsObject, With<GameObject>>,
) {

    if freecam.enabled || !mouse_button_input.just_pressed(MouseButton::Right) {
        return
    }

    let camera_pos = camera.get_single().unwrap();

    let look = camera_pos.rotation * Vec3::new(0.0, 0.0, -1.0);

    let cast = do_raycast(
        from_bevy_vec3(camera_pos.translation),
        from_bevy_vec3(look),
        MAX_INTERACTION_DISTANCE,
        &chunks,
        &blocks,
    );

    if cast.is_none() {
        return;
    }

    let ray = cast.unwrap();

    let Some(block_type) = inventory.selected_block_id() else {
        return
    };

    let pos = ray.block + ray.normal;

    // TODO: We could look this up instead
    let block_collider = Aabb::new(Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32), Vector3::new(1.0, 1.0, 1.0));

    // Ensure no gameobjects colliding
    for physics_object in game_objects.iter() {
        let aabb = physics_object.collider.offset(physics_object.position);

        let collides = aabb.aabb_collides(&block_collider);

        if collides {
            return
        }
    }

    // Locate chunk
    let (chunk_loc, inner_loc) = global_to_local_position(pos);

    // Try find chunk
    let Some(chunk) = chunks.chunks.get_mut(&chunk_loc) else {
        warn!("Tried to place block in unloaded chunk");
        return
    };

    if chunk.world.get(inner_loc) != 0 {
        // Trying to place a block on another block
        return
    }

    // Found chunk! Update block
    chunk.world.set(inner_loc, block_type);

    // Rerender
    rerender_chunk_event.send(RerenderChunkRequest {
        chunk: chunk_loc,
        context: RerenderChunkFlagContext::Surrounding,
    });

    debug!(
        "Updated [{}, {}, {}]",
        ray.block.x, ray.block.y, ray.block.z
    );

    inventory.take_selected_block();

    // Send network update
    send_packet.send(SendPacket(
        Protocol::PlaceBlock(PlaceBlock::new(pos.x, pos.y, pos.z)),
        UserId(0),
    ));
}
