use bevy::prelude::Entity;
use nalgebra::Vector3;
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
            PLAYER_POS[0].load(Ordering::Relaxed),
            PLAYER_POS[1].load(Ordering::Relaxed),
            PLAYER_POS[2].load(Ordering::Relaxed),
        )
        .cast::<f32>();

        Some(
            (self.chunk.cast::<f32>() - player)
                .magnitude()
                .total_cmp(&(other.chunk.cast::<f32>() - player).magnitude()),
        )
    }
}

impl Ord for MeshBuildEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let player = Vector3::new(
            PLAYER_POS[0].load(Ordering::Relaxed),
            PLAYER_POS[1].load(Ordering::Relaxed),
            PLAYER_POS[2].load(Ordering::Relaxed),
        )
        .cast::<f32>();

        (self.chunk.cast::<f32>() - player)
            .magnitude()
            .total_cmp(&(other.chunk.cast::<f32>() - player).magnitude())
    }
}
