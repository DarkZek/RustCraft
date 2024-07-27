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
                let mut size = 1.0;

                if system.entity_mapping.contains_key(&entity.id) {
                    warn!("Duplicate game_object attempted to spawn {:?}", entity.id);
                    return;
                }

                let mesh = if let GameObjectData::ItemDrop(item) = &entity.data {
                    let identifier = &item.item.identifier;
                    size = 0.2;
                    generate_item_mesh(identifier, &block_states, &item_states)
                } else {
                    let mut mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

                    let len = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len();

                    mesh.insert_attribute(ATTRIBUTE_WIND_STRENGTH, vec![0.0f32; len] as Vec<f32>);
                    mesh.insert_attribute(ATTRIBUTE_LIGHTING_COLOR, vec![[1.0_f32; 4]; len] as Vec<[f32; 4]>);

                    mesh
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
                    .insert(GameObject {
                        data: entity.data.clone()
                    })
                    .insert(MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        material: asset_service.translucent_texture_atlas_material.clone(),
                        ..default()
                    })
                    .id();

                system.entity_mapping.insert(entity.id, entity_id);

                info!("Entity spawned {:?}!", entity.id);
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
