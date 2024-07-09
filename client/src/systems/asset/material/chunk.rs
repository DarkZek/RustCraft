use crate::systems::chunk::builder::ATTRIBUTE_LIGHTING_COLOR;
use bevy::asset::Asset;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::{MeshVertexBufferLayout, MeshVertexBufferLayoutRef};
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
};

// This is the struct that will be passed to your shader
#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct ChunkMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/chunk_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/chunk_material.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/chunk_material_depth.wgsl".into()
    }
    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/chunk_material_depth.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/chunk_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_LIGHTING_COLOR.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
