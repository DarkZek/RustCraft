use bevy::prelude::*;
use crate::particle::Particle;

pub fn do_despawn(
    query: Query<(Entity, &Particle)>,
    mut commands: Commands,
    time: Res<Time>
) {

    let now = time.elapsed();

    for (entity, particle) in query.iter() {

        let lived = now - particle.created;

        if lived < particle.ttl {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
