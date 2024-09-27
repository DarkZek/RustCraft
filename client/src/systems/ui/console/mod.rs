mod ui;
mod executor;

use bevy::app::App;
use bevy::color::Color;
use bevy::log::Level;
use bevy::prelude::{Entity, Event, EventReader, Plugin, Query, ResMut, Resource, Startup, Update, Visibility};
use web_time::Instant;
use rc_networking::protocol::Protocol;
use rc_networking::types::ReceivePacket;
use crate::systems::ui::console::executor::{CommandExecuted, execute_commands};
use crate::systems::ui::console::ui::{handle_keyboard_input, setup_console_ui, update_ui};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_console_ui)
            .add_systems(Update, (
                handle_keyboard_input,
                update_ui,
                expire_old,
                execute_commands,
                listen_for_messages
            ))
            .add_event::<ConsoleLog>();
    }
}

#[derive(Event)]
pub struct ConsoleLog(pub String, pub Level);

const MAX_CONSOLE_HISTORY: usize = 8;
const CONSOLE_HISTORY_RETENTION_SECONDS: f32 = 12.0;
const MAX_CHAT_LENGTH: usize = 200;

struct HistoryItem {
    text_color: Color,
    background_color: Color,
    message: String,
    created_at: Instant,
    expired: bool
}

#[derive(Resource)]
pub struct ConsoleData {
    ui: Entity,
    text_prompt: Entity,
    pub capturing: bool,
    prompt_text: String,
    history: Vec<HistoryItem>,
    dirty: bool,
    text_history: Entity,
    text_history_children_texts: Vec<Entity>,
    text_history_children_items: Vec<Entity>,
    commands_executed: Vec<CommandExecuted>
}

impl ConsoleData {
    pub fn execute_command(&mut self, command: &str) {

        if !command.starts_with("/") {
            // Send message
            self.commands_executed.push(CommandExecuted::Message(command.to_string()));
            return
        }

        self.log(&format!("Executing Command <{}>", command));
    }

    pub fn log(&mut self, message: &str) {
        let item = HistoryItem {
            text_color: Color::WHITE,
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            message: message.to_string(),
            created_at: Instant::now(),
            expired: false,
        };
        self.history.insert(0, item);

        if self.history.len() > MAX_CONSOLE_HISTORY {
            self.history.pop();
        }

        self.dirty = true;
    }

    pub fn log_warn(&mut self, message: &str) {
        let item = HistoryItem {
            text_color: Color::WHITE,
            background_color: Color::srgba(1.0, 0.6, 0.3, 0.65),
            message: message.to_string(),
            created_at: Instant::now(),
            expired: false,
        };
        self.history.insert(0, item);

        if self.history.len() > MAX_CONSOLE_HISTORY {
            self.history.pop();
        }

        self.dirty = true;
    }

    pub fn log_error(&mut self, message: &str) {
        let item = HistoryItem {
            text_color: Color::WHITE,
            background_color: Color::srgba(0.8, 0.0, 0.0, 0.65),
            message: message.to_string(),
            created_at: Instant::now(),
            expired: false,
        };
        self.history.insert(0, item);

        if self.history.len() > MAX_CONSOLE_HISTORY {
            self.history.pop();
        }

        self.dirty = true;
    }

    pub fn log_with_style(&mut self, message: &str, text_color: Color, background_color: Color) {
        let item = HistoryItem {
            text_color,
            background_color,
            message: message.to_string(),
            created_at: Instant::now(),
            expired: false,
        };
        self.history.insert(0, item);

        if self.history.len() > MAX_CONSOLE_HISTORY {
            self.history.pop();
        }

        self.dirty = true;
    }

    pub fn capture(&mut self, query: &mut Query<&mut Visibility>) {
        self.capturing = true;
        *query.get_mut(self.ui).unwrap() = Visibility::Visible;
        self.dirty = true;
    }

    pub fn uncapture(&mut self, query: &mut Query<&mut Visibility>) {
        self.capturing = false;
        *query.get_mut(self.ui).unwrap() = Visibility::Hidden;
        self.dirty = true;
    }

    pub fn expire_old(&mut self) {
        for (i, item) in self.history.iter_mut().enumerate() {
            if !item.expired && item.created_at.elapsed().as_secs_f32() > CONSOLE_HISTORY_RETENTION_SECONDS {
                item.expired = true;
                self.dirty = true;
            }
        }
    }
}

fn expire_old(mut data: ResMut<ConsoleData>) {
    data.expire_old();
}

fn listen_for_messages(
    mut messages: EventReader<ReceivePacket>,
    mut data: ResMut<ConsoleData>,
    mut reader: EventReader<ConsoleLog>
) {
    for message in messages.read() {
        let Protocol::ChatSent(chat) = &message.0 else {
            continue
        };

        data.log(&chat.message);
    }

    for message in reader.read() {
        if message.1 == Level::ERROR {
            data.log_error(&message.0);
        } else if message.1 == Level::WARN {
            data.log_warn(&message.0);
        } else {
            data.log(&message.0);
        }
    }
}