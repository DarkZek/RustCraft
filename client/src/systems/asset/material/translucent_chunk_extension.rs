use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use crate::systems::chunk::builder::ATTRIBUTE_WIND_STRENGTH;

pub type TranslucentChunkMaterial = ExtendedMaterial<StandardMaterial, TranslucentChunkMaterialExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TranslucentChunkMaterialExtension {
    #[uniform(100)]
    pub time: f32,
}

impl MaterialExtension for TranslucentChunkMaterialExtension {
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
        let vertex_attribute_id = layout.0.attribute_ids()
            .iter().position(|row| {
            row.clone() == ATTRIBUTE_WIND_STRENGTH.id
        });

        if let Some(vertex_attribute_id_i) = vertex_attribute_id {
            let mut attribute_layout = layout.0.layout().attributes.get(vertex_attribute_id_i).unwrap().clone();

            attribute_layout.shader_location = 15;
            descriptor.vertex.buffers.get_mut(0).unwrap().attributes.push(attribute_layout);
        } else {
            panic!("Attribute ATTRIBUTE_WIND_STRENGTH not specified in a mesh")
        }

        descriptor.vertex.shader_defs
            .push("IS_TRANSLUCENT".into());
        descriptor.fragment.as_mut().unwrap().shader_defs
            .push("IS_TRANSLUCENT".into());
        Ok(())
    }
}