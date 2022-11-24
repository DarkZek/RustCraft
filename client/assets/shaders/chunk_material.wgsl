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

struct ChunkInput {
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: ChunkInput) -> @location(0) vec4<f32> {
    let output_color: vec4<f32> = material.color;

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