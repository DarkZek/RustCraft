use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Event, EventReader, EventWriter, Plugin, Resource, Update, World};
use nalgebra::Vector3;
use rc_networking::protocol::clientbound::chat::ChatSent;
use rc_networking::protocol::clientbound::game_object_moved::GameObjectMoved;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::constants::UserId;
use rc_shared::item::ItemStates;
use rc_shared::item::types::ItemStack;
use crate::game::entity::DirtyPosition;
use crate::game::inventory::Inventory;
use crate::game::transform::Transform;
use crate::game::world::data::WorldData;
use crate::game::world::WORLD_SPAWN_LOCATION;
use crate::transport::TransportSystem;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_commands);
        app.add_event::<ExecuteCommandRequest>();
    }
}

#[derive(Event, Clone)]
pub struct ExecuteCommandRequest {
    pub user_id: UserId,
    pub message: String
}

pub fn execute_commands(
    world: &mut World,
    state: &mut SystemState<(
        EventReader<ExecuteCommandRequest>,
        EventWriter<SendPacket>
    )>,
) {

    let (mut events, _) = state.get_mut(world);

    let events = events.read().map(|v| v.clone()).collect::<Vec<ExecuteCommandRequest>>();

    for command in events {

        if command.message.len() == 0 {
            continue;
        }

        let args = command.message.split(" ").map(|v| v.to_string()).collect::<Vec<String>>();

        let command_name = args.get(0).unwrap();

        let response = match command_name.as_str() {
            "spawn" => {
                parse_spawn(args, command.user_id, world)
            }
            "give" => {
                parse_give(args, command.user_id, world)
            }
            &_ => format!("Unknown command <{}>", command_name)
        };

        let (_, mut send_packet) = state.get_mut(world);
        send_packet.send(SendPacket(
            Protocol::ChatSent(ChatSent {
                message: response
            }),
            command.user_id
        ));
    }
}

fn parse_spawn(command: Vec<String>, user_id: UserId, world: &mut World) -> String {
    if command.len() != 1 {
        return format!("Incorrect arguments for command <{}>", command.get(0).unwrap());
    }

    let game_object_id = world.get_resource::<TransportSystem>().unwrap().clients.get(&user_id).unwrap().game_object_id.unwrap();
    let entity = *world.get_resource::<WorldData>().unwrap().game_objects_mapping.get(&game_object_id).unwrap();

    let mut transform = world.query::<&mut Transform>().get_mut(world, entity).unwrap();

    // Move player for all other connected clients
    transform.position = Vector3::new(WORLD_SPAWN_LOCATION.x, WORLD_SPAWN_LOCATION.y, WORLD_SPAWN_LOCATION.z);
    world.entity_mut(entity).insert(DirtyPosition);

    // Move player for player
    world.send_event(SendPacket(
        Protocol::GameObjectMoved(GameObjectMoved {
            entity: game_object_id,
            x: WORLD_SPAWN_LOCATION.x,
            y: WORLD_SPAWN_LOCATION.y,
            z: WORLD_SPAWN_LOCATION.z,
        }),
        user_id
    ));

    format!("Returned to spawn")
}

fn parse_give(command: Vec<String>, user_id: UserId, world: &mut World) -> String {
    if command.len() != 4 {
        return format!("Incorrect arguments for command <{}> Usage: /give DarkZek mcv3::ImprovisedFurnaceItem 64", command.get(0).unwrap());
    }

    let target_user = command.get(1).unwrap();
    let target_item = command.get(2).unwrap();
    let item_amount = command.get(3).unwrap();

    // Lookup player by name
    let user_id = {
        let mut user_id = None;
        for (uid, user) in &world.get_resource::<TransportSystem>().unwrap().clients {
            if &user.name == target_user {
                user_id = Some(uid);
                break;
            }
        }

        if let Some(user_id) = user_id {
            *user_id
        } else {
            return format!("Invalid user {}", target_user);
        }
    };

    // Get item amount
    let item_amount = {
        if let Ok(amount) = item_amount.parse::<u32>() {
            amount
        } else {
            return format!("Invalid amount: {}", item_amount);
        }
    };

    // Lookup item
    let item_stack = {
        let Some((_, item)) = world.get_resource::<ItemStates>().unwrap().get_by_id(target_item) else {
            return format!("Invalid item: {}", target_item);
        };

        ItemStack::new(
            item.clone(),
            item_amount
        )
    };

    // Fetch player inventory
    let game_object_id = world.get_resource::<TransportSystem>().unwrap().clients.get(&user_id).unwrap().game_object_id.unwrap();
    let entity = *world.get_resource::<WorldData>().unwrap().game_objects_mapping.get(&game_object_id).unwrap();
    let mut inventory = world.query::<&mut Inventory>().get_mut(world, entity).unwrap();

    inventory.dirty = true;
    inventory.push_item(item_stack);

    format!("Added {} {} to {}'s inventory", item_amount, target_item, target_user)
}