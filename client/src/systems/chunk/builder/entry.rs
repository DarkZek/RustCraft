use bevy::prelude::Entity;
use nalgebra::Vector3;
use rc_shared::CHUNK_SIZE;
use std::sync::atomic::{AtomicI32, Ordering};

// Approximate player position used to load closer chunks
pub static PLAYER_POS: [AtomicI32; 3] = [AtomicI32::new(0), AtomicI32::new(0), AtomicI32::new(0)];

#[derive(Eq)]
pub struct MeshBuildEntry {
    pub entity: Entity,
    pub chunk: Vector3<i32>,
}

impl PartialEq<Self> for MeshBuildEntry {
    fn eq(&self, other: &Self) -> bool {
        self.chunk == other.chunk
    }
}

impl PartialOrd<Self> for MeshBuildEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let player = Vector3::new(
            PLAYER_POS[0].load(Ordering::SeqCst),
            PLAYER_POS[1].load(Ordering::SeqCst),
            PLAYER_POS[2].load(Ordering::SeqCst),
        )
        .cast::<f32>();

        let self_player_dist = ((self.chunk.cast::<f32>() * CHUNK_SIZE as f32) - player).magnitude();
        let other_player_dist = ((other.chunk.cast::<f32>() * CHUNK_SIZE as f32) - player).magnitude();

        Some(
            // Find chunk with smallest distance to player
            self_player_dist.total_cmp(&other_player_dist).reverse()
        )
    }
}

impl Ord for MeshBuildEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let player = Vector3::new(
            PLAYER_POS[0].load(Ordering::SeqCst),
            PLAYER_POS[1].load(Ordering::SeqCst),
            PLAYER_POS[2].load(Ordering::SeqCst),
        )
        .cast::<f32>();

        let self_player_dist = ((self.chunk.cast::<f32>() * CHUNK_SIZE as f32) - player).magnitude();
        let other_player_dist = ((other.chunk.cast::<f32>() * CHUNK_SIZE as f32) - player).magnitude();

        (
            // Find chunk with smallest distance to player
            self_player_dist.total_cmp(&other_player_dist).reverse()
        )
    }
}
