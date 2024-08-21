mod sync;

use bevy::app::{App, FixedUpdate};
use bevy::prelude::{Component, Plugin};
use crate::game::entity::sync::sync_entities;

#[derive(Component)]
pub struct DirtyPosition;

#[derive(Component)]
pub struct DirtyRotation;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, sync_entities);
    }
}