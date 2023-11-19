use crate::game::entity::Entity;

use crate::systems::networking::NetworkingSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;

use nalgebra::Vector3;

use crate::state::AppState;
use crate::systems::physics::aabb::Aabb;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut transforms: Query<&mut Transform>,
    mut physics_objects: Query<&mut PhysicsObject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut system: ResMut<NetworkingSystem>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        match &event.0 {
            Protocol::UpdateLoading(update) => {
                if update.loading {
                    app_state.set(AppState::Connecting);
                } else {
                    app_state.set(AppState::InGame);
                }
            }
            Protocol::EntityMoved(update) => {
                if let Some(Ok(mut transform)) = system
                    .entity_mapping
                    .get(&update.entity)
                    .map(|v| physics_objects.get_mut(*v))
                {
                    transform.position.x = update.x;
                    transform.position.y = update.y;
                    transform.position.z = update.z;
                } else {
                    error!("Move event received before entity created");
                }
            }
            Protocol::EntityRotated(update) => {
                if let Some(Ok(mut transform)) = system
                    .entity_mapping
                    .get(&update.entity)
                    .map(|v| transforms.get_mut(*v))
                {
                    transform.rotation = Quat::from_xyzw(update.x, update.y, update.z, update.w);
                } else {
                    error!("Rotate event received before entity created");
                }
            }
            Protocol::SpawnEntity(entity) => {
                let entity_id = commands
                    .spawn(Transform::from_rotation(Quat::from_xyzw(
                        entity.rot[0],
                        entity.rot[1],
                        entity.rot[2],
                        entity.rot[3],
                    )))
                    .insert(PhysicsObject::new(
                        Vector3::new(entity.loc[0], entity.loc[1], entity.loc[2]),
                        Aabb::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 1.0)),
                    ))
                    .insert(Entity)
                    .insert(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
                        ..default()
                    })
                    .id();

                system.entity_mapping.insert(entity.id, entity_id);

                info!("Entity spawned {:?}!", entity.id);
            }
            Protocol::FullChunkUpdate(_) => {}
            Protocol::PartialChunkUpdate(_) => {}
            Protocol::DespawnEntity(packet) => {
                if let Some(entity) = system.entity_mapping.remove(&packet.entity) {
                    commands.entity(entity).despawn();
                }
            }
            other => {
                info!("Other {:?}", other);
            }
        }
    }
}
