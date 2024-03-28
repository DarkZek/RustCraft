#import bevy_pbr::mesh_view_bindings    view, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_bindings         mesh

#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::utils
#import bevy_pbr::shadows

#import bevy_pbr::gtao_utils gtao_multibounce
#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::mesh_functions as mesh_functions
#import bevy_pbr::mesh_functions::affine_to_square
#import bevy_pbr::view_transformations::position_world_to_clip
#import bevy_pbr::pbr_deferred_functions::deferred_output
#import bevy_pbr::pbr_functions::main_pass_post_lighting_processing

struct ChunkMaterial {
    color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: ChunkMaterial;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) lighting: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) lighting: vec4<f32>,
};

@vertex
fn vertex(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    let model = mesh_functions::get_model_matrix(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vertex.position);

    out.position = position_world_to_clip(out.world_position.xyz);

    out.uv = vertex.uv;

    let ambient = 0.06;
    out.lighting = vec4(vertex.lighting.xyz + ambient, 1.0);

    return out;

    //out.clip_position = view.view_proj * out.world_position;
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

    var pbr_input: pbr_types::PbrInput = pbr_types::pbr_input_new();
    pbr_input.material.base_color = output_color;
    pbr_input.material.reflectance = 0.01;
    pbr_input.material.flags = pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;
    pbr_input.material.perceptual_roughness = 1.0;
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = pbr_functions::prepare_world_normal(
         in.world_normal,
         false,
         true,
    );
    pbr_input.is_orthographic = false;

    let ssao = textureLoad(screen_space_ambient_occlusion_texture, vec2<i32>(in.frag_coord.xy), 0i).r;
    let ssao_multibounce = gtao_multibounce(ssao, pbr_input.material.base_color.rgb);
    pbr_input.specular_occlusion = min(1.0, ssao_multibounce);

    pbr_input.N = bevy_pbr::prepass_utils::prepass_normal(in.frag_coord, 0u);
    pbr_input.V = pbr_functions::calculate_view(in.world_position, false);

    #ifdef PREPASS_PIPELINE
        // write the gbuffer, lighting pass id, and optionally normal and motion_vector textures
        return deferred_output(in, pbr_input);
    #else

        var pbr_color = pbr_functions::apply_pbr_lighting(pbr_input);

        var color = main_pass_post_lighting_processing(pbr_input, pbr_color);

        return color;

    #endif
}