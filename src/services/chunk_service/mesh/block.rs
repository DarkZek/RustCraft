use crate::block::Block;
use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::chunk_service::mesh::culling::ViewableDirection;
use crate::services::chunk_service::mesh::{Vertex, ViewableDirectionBitMap};
use nalgebra::{Point3, Vector3};
use crate::services::chunk_service::chunk::Color;

pub fn draw_block(
    point: Point3<f32>,
    viewable: ViewableDirection,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    block: &Block,
    applied_color: Color
) {
    if viewable.has_flag(ViewableDirectionBitMap::Top) {
        draw_y_face(
            point.x,
            point.y - 1.0,
            point.z,
            vertices,
            indices,
            true,
            block.texture_atlas_lookups[0],
            applied_color
        );
    }

    if viewable.has_flag(ViewableDirectionBitMap::Bottom) {
        draw_y_face(
            point.x,
            point.y - 2.0,
            point.z,
            vertices,
            indices,
            false,
            block.texture_atlas_lookups[5],
            applied_color
        );
    }

    if viewable.has_flag(ViewableDirectionBitMap::Front) {
        draw_x_face(
            point.x,
            point.y - 2.0,
            point.z,
            vertices,
            indices,
            true,
            block.texture_atlas_lookups[1],
            applied_color
        );
    }

    if viewable.has_flag(ViewableDirectionBitMap::Back) {
        draw_x_face(
            point.x,
            point.y - 2.0,
            point.z + 1.0,
            vertices,
            indices,
            false,
            block.texture_atlas_lookups[3],
            applied_color
        );
    }

    if viewable.has_flag(ViewableDirectionBitMap::Left) {
        draw_z_face(
            point.x,
            point.y - 2.0,
            point.z,
            vertices,
            indices,
            true,
            block.texture_atlas_lookups[2],
            applied_color
        );
    }

    if viewable.has_flag(ViewableDirectionBitMap::Right) {
        draw_z_face(
            point.x + 1.0,
            point.y - 2.0,
            point.z as f32,
            vertices,
            indices,
            false,
            block.texture_atlas_lookups[4],
            applied_color
        );
    }
}

pub fn draw_y_face(
    x: f32,
    y: f32,
    z: f32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    top: bool,
    atlas: TextureAtlasIndex,
    applied_color: Color
) {
    let (start_atlas, end_atlas) = atlas;
    let normals = if top {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, -1.0, 0.0]
    };

    let starting_vertices = vertices.len() as u16;

    vertices.push(Vertex {
        position: [x, y, z],
        tex_coords: [start_atlas[0], end_atlas[1]],
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x + 1.0, y, z],
        tex_coords: end_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x, y, z + 1.0],
        tex_coords: start_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x + 1.0, y, z + 1.0],
        tex_coords: [end_atlas[0], start_atlas[1]],
        normals,
        applied_color
    });

    if top {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 1);
        indices.push(starting_vertices + 3);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 2);
    } else {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 1);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 2);
        indices.push(starting_vertices + 3);
    }
}

pub fn draw_x_face(
    x: f32,
    y: f32,
    z: f32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    forwards: bool,
    atlas: TextureAtlasIndex,
    applied_color: Color
) {
    let (start_atlas, end_atlas) = atlas;
    let normals = if forwards {
        [1.0, 0.0, 0.0]
    } else {
        [-1.0, 0.0, 0.0]
    };

    let starting_vertices = vertices.len() as u16;

    vertices.push(Vertex {
        position: [x, y, z],
        tex_coords: [start_atlas[0], end_atlas[1]],
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x + 1.0, y, z],
        tex_coords: end_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x, y + 1.0, z],
        tex_coords: start_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x + 1.0, y + 1.0, z],
        tex_coords: [end_atlas[0], start_atlas[1]],
        normals,
        applied_color
    });

    if forwards {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 1);
        indices.push(starting_vertices + 3);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 2);
    } else {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 1);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 2);
        indices.push(starting_vertices + 3);
    }
}

pub fn draw_z_face(
    x: f32,
    y: f32,
    z: f32,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    left: bool,
    atlas: TextureAtlasIndex,
    applied_color: Color
) {
    let (start_atlas, end_atlas) = atlas;
    let normals = if left {
        [0.0, 0.0, 1.0]
    } else {
        [0.0, 0.0, -1.0]
    };

    let starting_vertices = vertices.len() as u16;

    vertices.push(Vertex {
        position: [x, y, z],
        tex_coords: [start_atlas[0], end_atlas[1]],
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x, y + 1.0, z],
        tex_coords: start_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x, y, z + 1.0],
        tex_coords: end_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [x, y + 1.0, z + 1.0],
        tex_coords: [end_atlas[0], start_atlas[1]],
        normals,
        applied_color
    });

    if left {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 1);
        indices.push(starting_vertices + 3);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 2);
    } else {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 1);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 2);
        indices.push(starting_vertices + 3);
    }
}

pub fn draw_vertical_face(
    start_position: Point3<f32>,
    size: Vector3<f32>,
    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u16>,
    top: bool,
    atlas: TextureAtlasIndex,
    applied_color: Color
) {
    let (start_atlas, end_atlas) = atlas;
    let normals = if top {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, -1.0, 0.0]
    };

    let starting_vertices = vertices.len() as u16;

    vertices.push(Vertex {
        position: [start_position.x, start_position.y, start_position.z],
        tex_coords: [start_atlas[0], end_atlas[1]],
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [
            start_position.x + size.x,
            start_position.y,
            start_position.z,
        ],
        tex_coords: end_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [
            start_position.x,
            start_position.y,
            start_position.z + size.z,
        ],
        tex_coords: start_atlas,
        normals,
        applied_color
    });
    vertices.push(Vertex {
        position: [
            start_position.x + size.x,
            start_position.y,
            start_position.z + size.z,
        ],
        tex_coords: [end_atlas[0], start_atlas[1]],
        normals,
        applied_color
    });

    if top {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 1);
        indices.push(starting_vertices + 3);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 2);
    } else {
        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 3);
        indices.push(starting_vertices + 1);

        indices.push(starting_vertices + 0);
        indices.push(starting_vertices + 2);
        indices.push(starting_vertices + 3);
    }
}
