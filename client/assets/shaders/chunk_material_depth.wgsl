#import bevy_pbr::mesh_view_bindings    view, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_bindings         mesh
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::gtao_utils gtao_multibounce

struct ChunkMaterial {
    color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(3) world_position: vec4<f32>,
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.world_position = mesh.model * vec4(vertex.position, 1.0);
    out.clip_position = view.view_proj * out.world_position;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;

    return out;
}

@group(1) @binding(0)
var<uniform> material: ChunkMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::mesh_functions as mesh_functions

struct FragmentInput {
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(3) world_position: vec4<f32>,
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