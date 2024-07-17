use bevy::pbr::{ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexBufferLayoutRef, VertexAttributeDescriptor};
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError};
use crate::systems::chunk::builder::ATTRIBUTE_LIGHTING_COLOR;

pub type ChunkMaterial = ExtendedMaterial<StandardMaterial, ChunkMaterialExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct ChunkMaterialExtension {
    #[uniform(100)]
    pub time: f32,
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

    // fn deferred_vertex_shader() -> ShaderRef {
    //     "shaders/extended_material.wgsl".into()
    // }

    // fn specialize(
    //     pipeline: &MaterialExtensionPipeline,
    //     descriptor: &mut RenderPipelineDescriptor,
    //     layout: &MeshVertexBufferLayoutRef,
    //     key: MaterialExtensionKey<Self>
    // ) -> Result<(), SpecializedMeshPipelineError> {
    //     println!("{:?}", descriptor.vertex.buffers);
    //     let layout = layout.0.get_layout(&[
    //         VertexAttributeDescriptor::new(0, Mesh::ATTRIBUTE_POSITION.id, "position"),
    //         VertexAttributeDescriptor::new(1, Mesh::ATTRIBUTE_NORMAL.id, "normal"),
    //         VertexAttributeDescriptor::new(2, Mesh::ATTRIBUTE_UV_0.id, "uv0"),
    //         // VertexAttributeDescriptor::new(3, Mesh::ATTRIBUTE_UV_1.id, "uv1"),
    //         // VertexAttributeDescriptor::new(4, Mesh::ATTRIBUTE_TANGENT.id, "tangent"),
    //         // VertexAttributeDescriptor::new(5, Mesh::ATTRIBUTE_COLOR.id, "color"),
    //         // VertexAttributeDescriptor::new(6, Mesh::ATTRIBUTE_JOINT_INDEX.id, "joint_indices"),
    //         // VertexAttributeDescriptor::new(7, Mesh::ATTRIBUTE_JOINT_WEIGHT.id, "joint_weight"),
    //         VertexAttributeDescriptor::new(14, ATTRIBUTE_LIGHTING_COLOR.id, "lighting"),
    //     ])?;
    //     descriptor.vertex.buffers = vec![layout];
    //     Ok(())
    // }
}