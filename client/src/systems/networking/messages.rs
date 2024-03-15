use crate::game::entity::Entity;

use crate::systems::networking::NetworkingSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;

use nalgebra::Vector3;
use rc_shared::game_objects::GameObjectData;

use crate::state::AppState;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use rc_shared::aabb::Aabb;

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
                    error!("Move event received before game_object created");
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
                    error!("Rotate event received before game_object created");
                }
            }
            Protocol::SpawnGameObject(entity) => {
                let size = if entity.data == GameObjectData::Player { 1.0 } else { 0.2 };

                if system.entity_mapping.contains_key(&entity.id) {
                    warn!("Duplicate entity attempted to spawn {:?}", entity.id);
                    return;
                }

                let color = if entity.data == GameObjectData::Debug {
                    Color::rgb(0.3, 0.8, 0.3)
                } else {
                    Color::rgb(1.0, 0.3, 0.3)
                };

                let entity_id = commands
                    .spawn(Transform::from_rotation(Quat::from_xyzw(
                        entity.rot[0],
                        entity.rot[1],
                        entity.rot[2],
                        entity.rot[3],
                    )))
                    .insert(PhysicsObject::new(
                        Vector3::new(entity.loc[0], entity.loc[1], entity.loc[2]),
                        Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(size, size, size)),
                    ))
                    .insert(Entity)
                    .insert(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: size })),
                        material: materials.add(color.into()),
                        ..default()
                    })
                    .id();

                system.entity_mapping.insert(entity.id, entity_id);

                info!("Entity spawned {:?}!", entity.id);
            }
            Protocol::FullChunkUpdate(_) => {}
            Protocol::PartialChunkUpdate(_) => {}
            Protocol::DespawnGameObject(packet) => {
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
