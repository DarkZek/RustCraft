use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub type ChunkMaterial = ExtendedMaterial<StandardMaterial, ChunkMaterialExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct ChunkMaterialExtension {

}

impl MaterialExtension for ChunkMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
}