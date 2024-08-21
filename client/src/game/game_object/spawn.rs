use crate::game::entity::{GameObject};

use crate::systems::networking::NetworkingSystem;
use crate::systems::physics::PhysicsObject;
use bevy::prelude::*;
use bevy::math::prelude::Cuboid;
use nalgebra::Vector3;
use rc_shared::game_objects::GameObjectData;
use rc_shared::item::ItemStates;

use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use rc_shared::aabb::Aabb;
use rc_shared::block::BlockStates;
use crate::game::game_object::mesh::generate_item_mesh;
use crate::game::player::Player;
use crate::systems::asset::AssetService;
use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_WIND_STRENGTH};

pub fn messages_update(
    mut event_reader: EventReader<ReceivePacket>,
    mut transforms: Query<&mut Transform>,
    mut physics_objects: Query<&mut PhysicsObject>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    block_states: Res<BlockStates>,
    item_states: Res<ItemStates>,
    mut system: ResMut<NetworkingSystem>,
    asset_service: Res<AssetService>
) {
    for event in event_reader.read() {
        match &event.0 {
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
                if system.entity_mapping.contains_key(&entity.id) {
                    warn!("Duplicate game_object attempted to spawn {:?}", entity.id);
                    return;
                }

                let mut entity_commands = commands
                    .spawn(Transform::from_rotation(Quat::from_xyzw(
                        entity.rot[0],
                        entity.rot[1],
                        entity.rot[2],
                        entity.rot[3],
                    )));
                entity_commands.insert(GameObject {
                    data: entity.data.clone()
                });

                let mut gravity = false;
                let aabb = match &entity.data {
                    GameObjectData::ItemDrop(item) => {
                        let identifier = &item.item_stack.item.identifier;

                        entity_commands.insert(MaterialMeshBundle {
                            mesh: meshes.add(generate_item_mesh(identifier, &block_states, &item_states)),
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
                            let mut mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

                            let len = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len();

                            mesh.insert_attribute(ATTRIBUTE_WIND_STRENGTH, vec![0.0f32; len] as Vec<f32>);
                            mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, vec![[1.0_f32; 4]; len] as Vec<[f32; 4]>);

                            entity_commands.insert(MaterialMeshBundle {
                                mesh: meshes.add(mesh),
                                material: asset_service.translucent_texture_atlas_material.clone(),
                                ..default()
                            });
                        }

                        Aabb::new(
                            Vector3::new(-0.35, -1.7, -0.35),
                            Vector3::new(0.7, 1.85, 0.7),
                        )
                    }
                    _ => unimplemented!()
                };

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
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}
