//
// This file is responsible for providing functions to easily draw rects
//

use crate::services::asset_service::atlas::TextureAtlasIndex;
use crate::services::chunk_service::mesh::UIVertex;

/// Draw sprite from texture atlas. This is used for things like inventory screens and icons.
pub fn draw_sprite<'a>(
    vertices: &'a mut Vec<UIVertex>,
    indices: &'a mut Vec<u16>,
    pos: [f32; 2],
    scale: [f32; 2],
    texture: TextureAtlasIndex,
    color: Option<[f32; 4]>,
) {
    let color = color.unwrap_or([1.0, 1.0, 1.0, 1.0]);
    let vertices_count = vertices.len() as u16;

    vertices.push(UIVertex {
        position: [pos[0], pos[1]],
        tex_coords: texture.0.clone(),
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0], pos[1] + scale[1]],
        tex_coords: [texture.0[0], texture.1[1]],
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0] + scale[0], pos[1]],
        tex_coords: [texture.1[0], texture.0[1]],
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0] + scale[0], pos[1] + scale[1]],
        tex_coords: texture.1,
        color,
    });

    indices.push(vertices_count);
    indices.push(vertices_count + 1);
    indices.push(vertices_count + 3);

    indices.push(vertices_count);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 3);
}

/// Draw rectangle. This is used for things like backgrounds for text.
pub fn draw_rect<'a>(
    vertices: &'a mut Vec<UIVertex>,
    indices: &'a mut Vec<u16>,
    pos: [f32; 2],
    scale: [f32; 2],
    color: [f32; 4],
) {
    let vertices_count = vertices.len() as u16;

    vertices.push(UIVertex {
        position: [pos[0], pos[1]],
        tex_coords: [0.0, 0.0],
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0], pos[1] + scale[1]],
        tex_coords: [0.0, 0.0],
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0] + scale[0], pos[1]],
        tex_coords: [0.0, 0.0],
        color,
    });
    vertices.push(UIVertex {
        position: [pos[0] + scale[0], pos[1] + scale[1]],
        tex_coords: [0.0, 0.0],
        color,
    });

    indices.push(vertices_count);
    indices.push(vertices_count + 1);
    indices.push(vertices_count + 3);

    indices.push(vertices_count);
    indices.push(vertices_count + 2);
    indices.push(vertices_count + 3);
}
