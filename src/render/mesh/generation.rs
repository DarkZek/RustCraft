use crate::render::mesh::Vertex;

pub fn generate_terrain() -> (Vec<Vertex>, Vec<u16>) {
    (vec![
        Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0]},
        Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0]},
        Vertex { position: [0.5, 0.5, 0.0], tex_coords: [1.0, 1.0]},
        Vertex { position: [-0.5, 0.5, 0.0], tex_coords: [0.0, 1.0]},
    ],
    vec![
        2, 1, 0,
        3, 2, 0,
    ])
}