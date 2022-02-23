use crate::services::chunk_service::mesh::UIVertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, Device};

pub struct UIMeshData {
    pub total_vertices: Vec<UIVertex>,
    pub total_indices: Vec<u16>,
    pub total_vertex_buffer: Option<Buffer>,
    pub total_indices_buffer: Option<Buffer>,
}

impl UIMeshData {
    pub fn new() -> Self {
        UIMeshData {
            total_vertices: vec![],
            total_indices: vec![],
            total_vertex_buffer: None,
            total_indices_buffer: None,
        }
    }

    pub fn clear(&mut self) {
        self.total_indices.clear();
        self.total_vertices.clear();
    }

    pub fn build_buf(&mut self, device: &Device) {
        self.total_vertex_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Mesh Data Vertex Buffer"),
            contents: &bytemuck::cast_slice(&self.total_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }));

        self.total_indices_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: Some("UI Mesh Data Indices Buffer"),
            contents: &bytemuck::cast_slice(&self.total_indices),
            usage: wgpu::BufferUsages::INDEX,
        }));
    }
}
