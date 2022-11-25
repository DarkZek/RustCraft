#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

struct ChunkMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ChunkMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::mesh_functions

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) lighting: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) lighting: vec4<f32>
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.world_position = mesh_position_local_to_world(mesh.model, vertex.position);
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;
    out.lighting = vertex.lighting;

    return out;
}

struct FragmentInput {
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) lighting: vec4<f32>
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let output_color: vec4<f32> = material.color * in.lighting;

    let output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);

    if output_color.a == 0.0 {
        discard;
    }

    var input: PbrInput;
    input.material.base_color = vec4(1.0, 1.0, 1.0, 1.0);
    input.material.reflectance = 0.03;
    input.material.flags = STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;
    input.material.perceptual_roughness = 1.0;
    input.material.metallic = 0.01;
    input.material.alpha_cutoff = 0.5;
    input.occlusion = 0.0;
    input.frag_coord = in.frag_coord;
    input.world_position = in.world_position;
    input.world_normal = prepare_world_normal(
         in.world_normal,
         false,
         true,
    );
    input.is_orthographic = false;

    input.N = apply_normal_mapping(
        input.material.flags,
        input.world_normal,
        in.uv
    );
    input.V = calculate_view(in.world_position, false);

    let output = (pbr(input) * 0.5) + vec4(0.5);

    return output * output_color;
}