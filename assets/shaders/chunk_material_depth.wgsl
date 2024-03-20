#import bevy_pbr::mesh_view_bindings    view, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_bindings         mesh
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::view_transformations::position_world_to_clip

struct ChunkMaterial {
    color: vec4<f32>,
};

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    let model = mesh_functions::get_model_matrix(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vertex.position);

    out.position = position_world_to_clip(out.world_position.xyz);
    out.uv = vertex.uv;

    return out;
}

@group(2) @binding(0)
var<uniform> material: ChunkMaterial;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

struct FragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct FragmentOutput {
    @location(0) normal: vec4<f32>,
}

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    var out: FragmentOutput;

    if textureSample(base_color_texture, base_color_sampler, in.uv).a < 0.1 {
       discard;
    }

    out.normal = vec4(in.world_normal, 1.0);

    return out;
}