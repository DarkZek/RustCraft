use bevy::app::App;
use bevy::ecs::system::SystemState;
use bevy::prelude::{Event, EventReader, EventWriter, Plugin, Resource, Update, World};
use nalgebra::Vector3;
use rc_networking::protocol::clientbound::chat::ChatSent;
use rc_networking::protocol::clientbound::game_object_moved::GameObjectMoved;
use rc_networking::protocol::Protocol;
use rc_networking::types::SendPacket;
use rc_shared::constants::UserId;
use crate::game::entity::DirtyPosition;
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