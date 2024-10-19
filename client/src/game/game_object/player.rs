use bevy::ecs::system::EntityCommands;
use bevy::math::{EulerRot, Quat};
use bevy::prelude::{Assets, BuildChildren, Color, Component, default, Entity, GlobalTransform, Handle, InheritedVisibility, MaterialMeshBundle, Mesh, Query, Text, TextSection, TextStyle, Transform, Vec3, ViewVisibility, Visibility};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy_mod_billboard::BillboardTextBundle;
use nalgebra::Vector3;
use rc_shared::atlas::{TextureAtlasIndex};
use rc_shared::block::face::Face;
use rc_shared::chunk::LightingColor;
use rc_shared::viewable_direction::ViewableDirectionBitMap;
use crate::game::game_object::Rotatable;
use crate::systems::asset::atlas::atlas::TEXTURE_ATLAS;
use crate::systems::asset::material::translucent_chunk_extension::TranslucentChunkMaterial;
use crate::utils::mesh::draw_kit::DrawKit;

#[derive(Component)]
pub struct PlayerGameObject {
    parent_entity: Entity,
    head_entity: Entity,
    body_entity: Entity,
    left_leg_entity: Entity,
    right_leg_entity: Entity,
    left_arm_entity: Entity,
    right_arm_entity: Entity
}

impl Rotatable for PlayerGameObject {
    fn rotate(&self, yaw: f32, pitch: f32, transforms: &mut Query<&mut Transform>) {
        // Head transform
        let mut head_rot = transforms.get_mut(self.head_entity).unwrap();

        let (x, _, z) = head_rot.rotation.to_euler(EulerRot::YXZ);

        let y = pitch;

        head_rot.rotation = Quat::from_euler(EulerRot::YXZ, x, y, z);

        // Parent transform
        let mut parent_rot = transforms.get_mut(self.parent_entity).unwrap();

        let (_, y, z) = parent_rot.rotation.to_euler(EulerRot::YXZ);

        let x = yaw;

        parent_rot.rotation = Quat::from_euler(EulerRot::YXZ, x, y, z);
    }
}

pub fn get_player_model(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>,
    parent_entity: Entity,
    username: String
) {

    entity_commands.insert((
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default()
    ));

    let mut head_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_head(&mut entity_commands, meshes, handle.clone());
        head_entity = Some(entity_commands.id());
    });
    let mut body_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_body(&mut entity_commands, meshes, handle.clone());
        body_entity = Some(entity_commands.id());
    });
    let mut left_leg_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_leg(&mut entity_commands, meshes, handle.clone(), Vector3::new(0.0, 0.0, 0.0));
        left_leg_entity = Some(entity_commands.id());
    });
    let mut right_leg_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_leg(&mut entity_commands, meshes, handle.clone(), Vector3::new(0.2, 0.0, 0.0));
        right_leg_entity = Some(entity_commands.id());
    });
    let mut left_arm_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_arm_left(&mut entity_commands, meshes, handle.clone());
        left_arm_entity = Some(entity_commands.id());
    });
    let mut right_arm_entity = None;
    entity_commands.with_children(|child_builder| {
        let mut entity_commands = child_builder.spawn(());
        player_arm_right(&mut entity_commands, meshes, handle.clone());
        right_arm_entity = Some(entity_commands.id());
    });

    entity_commands.with_children(|child_builder| {
        child_builder.spawn(BillboardTextBundle {
            transform: Transform::from_translation(Vec3::new(0., 2.1, 0.))
                .with_scale(Vec3::splat(0.0085)),
            text: Text::from_sections([
                TextSection {
                    value: username,
                    style: TextStyle {
                        font_size: 40.0,
                        font: Default::default(),
                        color: Color::WHITE,
                    }
                }
            ]),
            ..default()
        });
    });

    entity_commands.insert(PlayerGameObject {
        head_entity: head_entity.unwrap(),
        body_entity: body_entity.unwrap(),
        left_leg_entity: left_leg_entity.unwrap(),
        right_leg_entity: right_leg_entity.unwrap(),
        left_arm_entity: left_arm_entity.unwrap(),
        right_arm_entity: right_arm_entity.unwrap(),
        parent_entity
    });
}

fn player_head(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>
) {
    let head_atlas = TEXTURE_ATLAS.get().index.get("game/player_head").unwrap().clone();

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let head_atlas_back = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*0.0,
        (1.0/3.0)*1.0,
        0.5,
        1.0
    ));

    // Back
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.3),
        &Face {
            top_left: Vector3::new(-0.3, 0.0, 0.0),
            top_right: Vector3::new(-0.3, 0.6, 0.0),
            bottom_left: Vector3::new(0.3, 0.0, 0.0),
            texture: head_atlas_back,
            normal: Vector3::new(0.0, 0.0, 1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_face = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*0.0,
        (1.0/3.0)*1.0,
        0.0,
        0.5
    ));

    // Face
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, -0.3),
        &Face {
            top_left: Vector3::new(0.3, 0.0, 0.0),
            top_right: Vector3::new(0.3, 0.6, 0.0),
            bottom_left: Vector3::new(-0.3, 0.0, 0.0),
            texture: head_atlas_face,
            normal: Vector3::new(0.0, 0.0, -1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_top = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
        0.5,
        1.0
    ));

    // Top
    draw_kit.draw_face(
        Vector3::new(0.0, 0.6, 0.0),
        &Face {
            top_left: Vector3::new(0.3, 0.0, 0.3),
            top_right: Vector3::new(-0.3, 0.0, 0.3),
            bottom_left: Vector3::new(0.3, 0.0, -0.3),
            texture: head_atlas_top,
            normal: Vector3::new(0.0, 1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_bottom = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*2.0,
        (1.0/3.0)*3.0,
        0.5,
        1.0
    ));

    // Bottom
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(-0.3, 0.0, 0.3),
            top_right: Vector3::new(0.3, 0.0, 0.3),
            bottom_left: Vector3::new(-0.3, 0.0, -0.3),
            texture: head_atlas_bottom,
            normal: Vector3::new(0.0, -1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_left = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
        0.0,
        0.5
    )).flipped();

    // Left
    draw_kit.draw_face(
        Vector3::new(-0.3, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, -0.3),
            top_right: Vector3::new(0.0, 0.6, -0.3),
            bottom_left: Vector3::new(0.0, 0.0, 0.3),
            texture: head_atlas_left,
            normal: Vector3::new(-1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_right = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
        0.0,
        0.5
    ));

    // Right
    draw_kit.draw_face(
        Vector3::new(0.3, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, 0.3),
            top_right: Vector3::new(0.0, 0.6, 0.3),
            bottom_left: Vector3::new(0.0, 0.0, -0.3),
            texture: head_atlas_right,
            normal: Vector3::new(1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    draw_kit.apply_mesh(&mut mesh);

    entity_commands.insert(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 1.25, 0.0)),
        ..default()
    });
}

fn player_body(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>
) {
    let head_atlas = TEXTURE_ATLAS.get().index.get("game/player_body").unwrap().clone();

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let head_atlas_back = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*4.0,
        (1.0/6.0)*6.0,
        (1.0/4.0)*1.0,
        (1.0/4.0)*4.0,
    ));

    // Back
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.1),
        &Face {
            top_left: Vector3::new(-0.2, 0.0, 0.0),
            top_right: Vector3::new(-0.2, 0.6, 0.0),
            bottom_left: Vector3::new(0.2, 0.0, 0.0),
            texture: head_atlas_back,
            normal: Vector3::new(0.0, 0.0, 1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_face = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*1.0,
        (1.0/6.0)*3.0,
        (1.0/4.0)*1.0,
        (1.0/4.0)*4.0,
    ));

    // Face
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, -0.1),
        &Face {
            top_left: Vector3::new(0.2, 0.0, 0.0),
            top_right: Vector3::new(0.2, 0.6, 0.0),
            bottom_left: Vector3::new(-0.2, 0.0, 0.0),
            texture: head_atlas_face,
            normal: Vector3::new(0.0, 0.0, -1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_top = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*1.0,
        (1.0/6.0)*3.0,
        (1.0/4.0)*0.0,
        (1.0/4.0)*1.0,
    ));

    // Top
    draw_kit.draw_face(
        Vector3::new(0.0, 0.6, 0.0),
        &Face {
            top_left: Vector3::new(0.2, 0.0, 0.1),
            top_right: Vector3::new(-0.2, 0.0, 0.1),
            bottom_left: Vector3::new(0.2, 0.0, -0.1),
            texture: head_atlas_top,
            normal: Vector3::new(0.0, 1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_bottom = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*4.0,
        (1.0/6.0)*6.0,
        (1.0/4.0)*0.0,
        (1.0/4.0)*1.0,
    ));

    // Bottom
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(-0.2, 0.0, 0.1),
            top_right: Vector3::new(0.2, 0.0, 0.1),
            bottom_left: Vector3::new(-0.2, 0.0, -0.1),
            texture: head_atlas_bottom,
            normal: Vector3::new(0.0, -1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_left = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*0.0,
        (1.0/6.0)*1.0,
        (1.0/4.0)*1.0,
        (1.0/4.0)*4.0,
    )).flipped();

    // Left
    draw_kit.draw_face(
        Vector3::new(-0.2, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, -0.1),
            top_right: Vector3::new(0.0, 0.6, -0.1),
            bottom_left: Vector3::new(0.0, 0.0, 0.1),
            texture: head_atlas_left,
            normal: Vector3::new(-1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_right = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/6.0)*3.0,
        (1.0/6.0)*4.0,
        (1.0/4.0)*1.0,
        (1.0/4.0)*4.0,
    ));

    // Right
    draw_kit.draw_face(
        Vector3::new(0.2, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, 0.1),
            top_right: Vector3::new(0.0, 0.6, 0.1),
            bottom_left: Vector3::new(0.0, 0.0, -0.1),
            texture: head_atlas_right,
            normal: Vector3::new(1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    draw_kit.apply_mesh(&mut mesh);

    entity_commands.insert(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.65, 0.0)),
        ..default()
    });
}

fn  player_leg(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>,
    offset: Vector3<f32>
) {
    let head_atlas = TEXTURE_ATLAS.get().index.get("game/player_legs").unwrap().clone();

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let head_atlas_back = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*0.0,
        (1.0/5.0)*1.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Back
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.1) + offset,
        &Face {
            top_left: Vector3::new(-0.2, -0.6, 0.0),
            top_right: Vector3::new(-0.2, 0.0, 0.0),
            bottom_left: Vector3::new(0.0, -0.6, 0.0),
            texture: head_atlas_back,
            normal: Vector3::new(0.0, 0.0, 1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_face = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*1.0,
        (1.0/5.0)*2.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Face
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, -0.1) + offset,
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.0),
            top_right: Vector3::new(0.0, 0.0, 0.0),
            bottom_left: Vector3::new(-0.2, -0.6, 0.0),
            texture: head_atlas_face,
            normal: Vector3::new(0.0, 0.0, -1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_top = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*1.0,
    ));

    // Top
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0) + offset,
        &Face {
            top_left: Vector3::new(0.0, 0.0, 0.1),
            top_right: Vector3::new(-0.2, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, 0.0, -0.1),
            texture: head_atlas_top,
            normal: Vector3::new(0.0, 1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_bottom = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
    ));

    // Bottom
    draw_kit.draw_face(
        Vector3::new(0.0, -0.6, 0.0) + offset,
        &Face {
            top_left: Vector3::new(0.0, 0.0, -0.1),
            top_right: Vector3::new(-0.2, 0.0, -0.1),
            bottom_left: Vector3::new(0.0, 0.0, 0.1),
            texture: head_atlas_bottom,
            normal: Vector3::new(0.0, -1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_left = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*2.0,
        (1.0/5.0)*3.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    )).flipped();

    // Left
    draw_kit.draw_face(
        Vector3::new(-0.2, 0.0, 0.0) + offset,
        &Face {
            top_left: Vector3::new(0.0, -0.6, -0.1),
            top_right: Vector3::new(0.0, 0.0, -0.1),
            bottom_left: Vector3::new(0.0, -0.6, 0.1),
            texture: head_atlas_left,
            normal: Vector3::new(-1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_right = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*3.0,
        (1.0/5.0)*4.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Right
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0) + offset,
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.1),
            top_right: Vector3::new(0.0, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, -0.6, -0.1),
            texture: head_atlas_right,
            normal: Vector3::new(1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    draw_kit.apply_mesh(&mut mesh);

    entity_commands.insert(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.65, 0.0)),
        ..default()
    });
}

fn player_arm_right(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>
) {
    let head_atlas = TEXTURE_ATLAS.get().index.get("game/player_arms").unwrap().clone();

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let head_atlas_back = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*0.0,
        (1.0/5.0)*1.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Back
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.1),
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.0),
            top_right: Vector3::new(0.0, 0.0, 0.0),
            bottom_left: Vector3::new(0.2, -0.6, 0.0),
            texture: head_atlas_back,
            normal: Vector3::new(0.0, 0.0, 1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_face = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*1.0,
        (1.0/5.0)*2.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Face
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, -0.1),
        &Face {
            top_left: Vector3::new(0.2, -0.6, 0.0),
            top_right: Vector3::new(0.2, 0.0, 0.0),
            bottom_left: Vector3::new(0.0, -0.6, 0.0),
            texture: head_atlas_face,
            normal: Vector3::new(0.0, 0.0, -1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_top = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*1.0,
    ));

    // Top
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.2, 0.0, 0.1),
            top_right: Vector3::new(0.0, 0.0, 0.1),
            bottom_left: Vector3::new(0.2, 0.0, -0.1),
            texture: head_atlas_top,
            normal: Vector3::new(0.0, 1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_bottom = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
    ));

    // Bottom
    draw_kit.draw_face(
        Vector3::new(0.0, -0.6, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, 0.1),
            top_right: Vector3::new(0.2, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, 0.0, -0.1),
            texture: head_atlas_bottom,
            normal: Vector3::new(0.0, -1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_left = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*2.0,
        (1.0/5.0)*3.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    )).flipped();

    // Left
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, -0.6, -0.1),
            top_right: Vector3::new(0.0, 0.0, -0.1),
            bottom_left: Vector3::new(0.0, -0.6, 0.1),
            texture: head_atlas_left,
            normal: Vector3::new(-1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_right = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*3.0,
        (1.0/5.0)*4.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Right
    draw_kit.draw_face(
        Vector3::new(0.2, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.1),
            top_right: Vector3::new(0.0, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, -0.6, -0.1),
            texture: head_atlas_right,
            normal: Vector3::new(1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    draw_kit.apply_mesh(&mut mesh);

    entity_commands.insert(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: handle.clone(),
        transform: Transform::from_translation(Vec3::new(0.2, 1.25, 0.0)),
        ..default()
    });
}

fn player_arm_left(
    entity_commands: &mut EntityCommands,
    meshes: &mut Assets<Mesh>,
    handle: Handle<TranslucentChunkMaterial>,
) {
    let head_atlas = TEXTURE_ATLAS.get().index.get("game/player_arms").unwrap().clone();

    let mut draw_kit = DrawKit::new().with_wind_strength();

    let head_atlas_back = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*0.0,
        (1.0/5.0)*1.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Back
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.1),
        &Face {
            top_left: Vector3::new(-0.2, -0.6, 0.0),
            top_right: Vector3::new(-0.2, 0.0, 0.0),
            bottom_left: Vector3::new(0.0, -0.6, 0.0),
            texture: head_atlas_back,
            normal: Vector3::new(0.0, 0.0, 1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Back,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_face = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*1.0,
        (1.0/5.0)*2.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Face
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, -0.1),
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.0),
            top_right: Vector3::new(0.0, 0.0, 0.0),
            bottom_left: Vector3::new(-0.2, -0.6, 0.0),
            texture: head_atlas_face,
            normal: Vector3::new(0.0, 0.0, -1.0),
            edge: false,
            direction: ViewableDirectionBitMap::Front,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_top = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*1.0,
    ));

    // Top
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, 0.1),
            top_right: Vector3::new(-0.2, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, 0.0, -0.1),
            texture: head_atlas_top,
            normal: Vector3::new(0.0, 1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Top,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_bottom = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*4.0,
        (1.0/5.0)*5.0,
        (1.0/3.0)*1.0,
        (1.0/3.0)*2.0,
    ));

    // Bottom
    draw_kit.draw_face(
        Vector3::new(0.0, -0.6, 0.0),
        &Face {
            top_left: Vector3::new(0.0, 0.0, -0.1),
            top_right: Vector3::new(-0.2, 0.0, -0.1),
            bottom_left: Vector3::new(0.0, 0.0, 0.1),
            texture: head_atlas_bottom,
            normal: Vector3::new(0.0, -1.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Bottom,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_left = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*2.0,
        (1.0/5.0)*3.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    )).flipped();

    // Left
    draw_kit.draw_face(
        Vector3::new(-0.2, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, -0.6, -0.1),
            top_right: Vector3::new(0.0, 0.0, -0.1),
            bottom_left: Vector3::new(0.0, -0.6, 0.1),
            texture: head_atlas_left,
            normal: Vector3::new(-1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Left,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let head_atlas_right = head_atlas.sub_index(&TextureAtlasIndex::new(
        (1.0/5.0)*3.0,
        (1.0/5.0)*4.0,
        (1.0/3.0)*0.0,
        (1.0/3.0)*3.0,
    ));

    // Right
    draw_kit.draw_face(
        Vector3::new(0.0, 0.0, 0.0),
        &Face {
            top_left: Vector3::new(0.0, -0.6, 0.1),
            top_right: Vector3::new(0.0, 0.0, 0.1),
            bottom_left: Vector3::new(0.0, -0.6, -0.1),
            texture: head_atlas_right,
            normal: Vector3::new(1.0, 0.0, 0.0),
            edge: false,
            direction: ViewableDirectionBitMap::Right,
            wind_strengths: None,
        },
        LightingColor::full()
    );

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    draw_kit.apply_mesh(&mut mesh);

    entity_commands.insert(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: handle.clone(),
        transform: Transform::from_translation(Vec3::new(-0.2, 1.25, 0.0)),
        ..default()
    });
}