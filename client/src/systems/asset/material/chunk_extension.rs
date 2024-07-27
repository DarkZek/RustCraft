use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use crate::systems::chunk::builder::{ATTRIBUTE_LIGHTING_COLOR, ATTRIBUTE_WIND_STRENGTH};

pub type ChunkMaterial = ExtendedMaterial<StandardMaterial, ChunkMaterialExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct ChunkMaterialExtension {

}

impl MaterialExtension for ChunkMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/extended_material.wgsl".into()
    }
    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/extended_material_prepass.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {

        add_vertex_extension(layout, descriptor, ATTRIBUTE_LIGHTING_COLOR, 14);

        Ok(())
    }
}

pub fn add_vertex_extension(
    layout: &MeshVertexBufferLayoutRef,
    descriptor: &mut RenderPipelineDescriptor,
    attribute: MeshVertexAttribute,
    shader_location: u32
) {
    let vertex_attribute_id = layout.0.attribute_ids()
        .iter().position(|row| {
        row.clone() == attribute.id
    });

    if let Some(vertex_attribute_id_i) = vertex_attribute_id {
        let mut attribute_layout = layout.0.layout().attributes.get(vertex_attribute_id_i).unwrap().clone();

        attribute_layout.shader_location = shader_location;
        descriptor.vertex.buffers.get_mut(0).unwrap().attributes.push(attribute_layout);
    } else {
        panic!("Attribute ATTRIBUTE_LIGHTING_COLOR not specified in a mesh")
    }
}