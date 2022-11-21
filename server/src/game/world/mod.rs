use crate::game::world::data::WorldData;
use crate::{detect_shutdowns, App, AppExit};
use bevy::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

pub mod data;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(save_world);
    }
}

fn save_world(mut world: Res<WorldData>, mut bevy_shutdown: EventReader<AppExit>) {
    if bevy_shutdown.is_empty() {
        return;
    }

    info!("Saving world...");
    fs::create_dir_all("./world/").unwrap();

    for (pos, chunk) in &world.chunks {
        let file = File::create(format!(
            "./world/{:08x}{:08x}{:08x}.chunk",
            pos.x, pos.y, pos.z
        ))
        .unwrap();

        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, chunk).unwrap();
    }

    info!("Saved world.");
}
