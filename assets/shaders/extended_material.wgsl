#import bevy_pbr::{
    pbr_deferred_functions::deferred_output,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    mesh_functions,
    skinning,
    view_transformations::position_world_to_clip,
}
#import "shaders/wind.wgsl" get_wind

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

#import "shaders/material_pass.wgsl"::{ CustomVertexInput, CustomVertexOutput }

// TODO: Document which parts are copied from Bevy

struct ChunkExtendedMaterial {
#ifdef IS_TRANSLUCENT
    time: f32,
#endif
    ambient_strength: f32,
}

@group(2) @binding(100)
var<uniform> chunk_material: ChunkExtendedMaterial;

@vertex
fn vertex(vertex_no_morph: CustomVertexInput) -> CustomVertexOutput {
    var out: CustomVertexOutput;

    #ifdef SKINNED
        var world_from_local = skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
    #else
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416 .
        var world_from_local = mesh_functions::get_world_from_local(vertex_no_morph.instance_index);
    #endif

    #ifdef VERTEX_NORMALS
    #ifdef SKINNED
        out.world_normal = skinning::skin_normals(world_from_local, vertex_no_morph.normal);
    #else
        out.world_normal = mesh_functions::mesh_normal_local_to_world(
            vertex_no_morph.normal,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif
    #endif

    #ifdef VERTEX_POSITIONS
        out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex_no_morph.position, 1.0));

    #ifdef IS_TRANSLUCENT
        out.world_position += get_wind(out.world_position, chunk_material.time) * vertex_no_morph.wind_strength;
    #endif // IS_TRANSLUCENT

        out.position = position_world_to_clip(out.world_position.xyz);
    #endif

    #ifdef VERTEX_UVS_A
        out.uv = vertex_no_morph.uv;
    #endif
    #ifdef VERTEX_UVS_B
        out.uv_b = vertex_no_morph.uv_b;
    #endif

    #ifdef VERTEX_TANGENTS
        out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
            world_from_local,
            vertex.tangent,
            // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
            // See https://github.com/gfx-rs/naga/issues/2416
            vertex_no_morph.instance_index
        );
    #endif

    #ifdef VERTEX_COLORS
        out.color = vertex.color;
    #endif

    #ifdef VERTEX_OUTPUT_INSTANCE_INDEX
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        out.instance_index = vertex_no_morph.instance_index;
    #endif

    out.lighting = vec4(vertex_no_morph.lighting.xyz, 1.0) + vec4(chunk_material.ambient_strength);

    return out;
}

@fragment
fn fragment(
    in: CustomVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // TODO: Figure out how to do this without copying a bunch of data
    var input: VertexOutput;
    input.position = in.position;
    input.world_position = in.world_position;
    input.world_normal = in.world_normal;
#ifdef VERTEX_UVS_A
    input.uv = in.uv;
#endif // VERTEX_UVS_A
    input.instance_index = in.instance_index;

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(input, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    out.color *= in.lighting * 2.0;

    return out;
}