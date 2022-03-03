use crate::render::camera::Camera;
use crate::render::pass::outline::BoxOutline;
use crate::services::chunk_service::chunk::{ChunkData, ChunkEntityLookup};
use crate::world::raycast::Raycast;
use crate::world::WorldChunks;
use nalgebra::Vector3;
use specs::{
    Builder, Entities, Entity, Read, ReadStorage, System, World, WorldExt, Write, WriteStorage,
};
use std::sync::Arc;
use wgpu::Device;

pub struct PlayerSelectedBlockUpdateSystem;

pub struct PlayerSelectedBlockUpdateSystemData {
    position: Option<Vector3<f32>>,
    look_view: Option<Entity>,
}

impl Default for PlayerSelectedBlockUpdateSystemData {
    fn default() -> Self {
        PlayerSelectedBlockUpdateSystemData {
            position: None,
            look_view: None,
        }
    }
}

impl PlayerSelectedBlockUpdateSystemData {
    pub fn update_position(&mut self) {}

    pub fn new(universe: &mut World, device: Arc<Device>) -> PlayerSelectedBlockUpdateSystemData {
        let mut box_outline = BoxOutline::new(
            Vector3::new(-2.0, 69.0, 2.0),
            Vector3::new(1.0, 1.0, 1.0),
            [0.0; 4],
            device.clone(),
        );
        box_outline.build();
        let outline = universe.create_entity().with(box_outline).build();

        PlayerSelectedBlockUpdateSystemData {
            position: None,
            look_view: Some(outline),
        }
    }
}

impl<'a> System<'a> for PlayerSelectedBlockUpdateSystem {
    type SystemData = (
        Read<'a, Camera>,
        Write<'a, PlayerSelectedBlockUpdateSystemData>,
        ReadStorage<'a, ChunkData>,
        Write<'a, ChunkEntityLookup>,
        WriteStorage<'a, BoxOutline>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (camera, mut instance, chunks, lookup, mut outlines, entities): Self::SystemData,
    ) {
        //let pos =
        let world_chunks = WorldChunks::new(&chunks, &lookup);

        let direction = camera.look_direction();

        let cast = world_chunks.do_raycast(Raycast::new(camera.eye.coords, direction, 100));

        if let Some(entity) = instance.look_view {
            let outline = outlines.get_mut(entity).unwrap();

            let pos = if let Some(val) = cast {
                nalgebra::convert::<Vector3<i64>, Vector3<f32>>(val)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            outline.move_to(pos);
            outline.build();
        }
    }
}
