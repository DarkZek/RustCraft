#import bevy_pbr::mesh_view_bindings    view, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_bindings         mesh

#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows

#import bevy_pbr::gtao_utils gtao_multibounce
#import bevy_pbr::prepass_utils
#import bevy_pbr::pbr_functions as pbr_functions

struct ChunkMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ChunkMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::mesh_functions as mesh_functions

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

    out.world_position = mesh.model * vertex.position;
    out.clip_position = view.view_proj * out.world_position;
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal);
    out.uv = vertex.uv;

    let ambient = 0.02;
    out.lighting = vec4(vertex.lighting.xyz + ambient, 1.0);

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
    var output_color: vec4<f32> = material.color;

    output_color *= textureSample(base_color_texture, base_color_sampler, in.uv);

    if output_color.a < 0.1 {
       discard;
    }

    var input: pbr_functions::PbrInput = pbr_functions::pbr_input_new();
    input.material.base_color = vec4(1.0, 1.0, 1.0, 1.0);
    input.material.reflectance = 0.03;
    input.material.flags = pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;
    input.material.perceptual_roughness = 1.0;
    input.material.metallic = 0.01;
    input.material.alpha_cutoff = 0.5;
    input.frag_coord = in.frag_coord;
    input.world_position = in.world_position;
    input.world_normal = pbr_functions::prepare_world_normal(
         in.world_normal,
         false,
         true,
    );
    input.is_orthographic = false;

    let ssao = textureLoad(screen_space_ambient_occlusion_texture, vec2<i32>(in.frag_coord.xy), 0i).r;
    let ssao_multibounce = gtao_multibounce(ssao, input.material.base_color.rgb);
    input.occlusion = min(vec3(1.0), ssao_multibounce);

    input.N = bevy_pbr::prepass_utils::prepass_normal(in.frag_coord, 0u);
    input.V = pbr_functions::calculate_view(in.world_position, false);

    var pbr_color = pbr_functions::pbr(input);

    return pbr_color * in.lighting * output_color;
}