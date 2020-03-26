use specs::{Component, VecStorage, World, WorldExt, Builder, RunNow};
use crate::entity::ecs::physics::{PhysicsSystem};

pub mod physics;
pub mod movement;

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Rotation {
    s: f32,
    xi: f32,
    yj: f32,
    zk: f32,
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Player {
    local: bool
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

pub fn create_store() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Rotation>();
    world.register::<Player>();

    world.create_entity()
        .with(Position { x: 0.0, y: 0.0, z: 0.0})
        .with(Velocity { x: 0.0, y: 1.0, z: 0.0})
        .with(Rotation {
            s: 0.0,
            xi: 0.0,
            yj: 0.0,
            zk: 0.0
        })
        .with(Player{
            local: true
        }).build();

    let mut store = PhysicsSystem;
    store.run_now(&world);
    world.maintain();

    world
}