use bevy::prelude::{Fixed, Res, ResMut, Resource, Time};
use web_time::{Instant, UNIX_EPOCH};
use rc_shared::PHYSICS_SYNC_RATE_SECONDS;

#[derive(Resource)]
pub struct PhysicsInterpolation {
    pub amount: f32
}

pub fn calculate_interpolation_amount(
    mut interpolation: ResMut<PhysicsInterpolation>,
    time: Res<Time<Fixed>>
) {

    interpolation.amount = time.overstep_fraction();
}