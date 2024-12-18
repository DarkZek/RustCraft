use crate::game::entity::{GameObject};

use crate::systems::networking::NetworkingSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use nalgebra::{Quaternion, Vector3};
use rc_shared::game_objects::GameObjectData;
use rc_shared::item::ItemStates;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use rc_shared::aabb::Aabb;
use rc_shared::block::BlockStates;
use crate::game::game_object::mesh::generate_item_mesh;
use crate::game::game_object::player::{get_player_model, PlayerGameObject};
use crate::game::game_object::Rotatable;
use crate::game::player::Player;
use crate::systems::asset::AssetService;

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut physics_objects: Query<&mut PhysicsObject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    item_states: Res<ItemStates>,
    mut system: ResMut<NetworkingSystem>,
    asset_service: Res<AssetService>
) {
    for event in event_reader.read() {
        match &event.0 {
            Protocol::GameObjectMoved(update) => {
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
            Protocol::GameObjectRotated(update) => {
                let Some(entity) = system
                    .entity_mapping
                    .get(&update.entity) else {
                    continue
                };

                let Ok(mut physics) = physics_objects.get_mut(*entity) else {
                    warn!("Received GameObjectRotated event for unspawned game object");
                    continue
                };

                physics.rotation = Quaternion::new(update.w, update.x, update.y, update.z);
            }
            Protocol::SpawnGameObject(entity) => {
                if system.entity_mapping.contains_key(&entity.id) {
                    warn!("Duplicate game_object attempted to spawn {:?}", entity.id);
                    return;
                }

                let mut entity_commands = commands
                    .spawn(Transform::default());
                entity_commands.insert(GameObject {
                    data: entity.data.clone()
                });

                let mut gravity = false;
                let aabb = match &entity.data {
                    GameObjectData::ItemDrop(item) => {
                        let identifier = &item.item_stack.item.identifier;

                        entity_commands.insert(MaterialMeshBundle {
                            mesh: meshes.add(generate_item_mesh(identifier, &item_states)),
                            material: asset_service.translucent_texture_atlas_material.clone(),
                            ..default()
                        });

                        Aabb::new(
                            Vector3::new(-0.1, -0.1, -0.1),
                            Vector3::new(0.2, 0.2, 0.2)
                        )
                    }
                    GameObjectData::Player(player_data) => {

                        let is_self_player = player_data.user_id == system.user_id;

                        if is_self_player {
                            entity_commands.insert(Player::new());
                            gravity = true;
                        } else {
                            let entity = entity_commands.id();
                            get_player_model(
                                &mut entity_commands,
                                &mut meshes,
                                asset_service.translucent_texture_atlas_material.clone(),
                                entity,
                                player_data.username.clone()
                            );
                        }

                        Aabb::new(
                            Vector3::new(-0.35, 0.0, -0.35),
                            Vector3::new(0.7, 1.85, 0.7),
                        )
                    }
                    _ => unimplemented!()
                };

                // TODO: Include rotation
                let mut physics = PhysicsObject::new(
                    Vector3::new(entity.loc[0], entity.loc[1], entity.loc[2]),
                    aabb,
                );
                physics.gravity = gravity;
                entity_commands.insert(physics);

                system.entity_mapping.insert(entity.id, entity_commands.id());
            }
            Protocol::DespawnGameObject(packet) => {
                if let Some(entity) = system.entity_mapping.remove(&packet.entity) {
                    commands.entity(entity).despawn_recursive();
                } else {
                    warn!("Attempted to despawn entity that was not spawned {:?}", packet);
                }
            }
            _ => {}
        }
    }
}
