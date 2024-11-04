mod model;

use std::time::Duration;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Particle {
    /// How long the particle will live for
    pub ttl: Duration,
    /// The timestamp of when the particle was spawned
    pub created: Duration
}