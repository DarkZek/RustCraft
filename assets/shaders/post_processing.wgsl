#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var depth_texture: texture_depth_2d;
@group(0) @binding(2) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(3) var<uniform> settings: PostProcessSettings;
@group(0) @binding(4) var<uniform> view: View;
//
//// Bevy is an absolute pain in the ass to import its real mesh shader bindings to so just manually do define here
fn position_ndc_to_world(ndc_pos: vec3<f32>) -> vec3<f32> {
    let world_pos = view.world_from_clip * vec4(ndc_pos, 1.0);
    return world_pos.xyz / world_pos.w;
}
/// Convert uv [0.0 .. 1.0] coordinate to ndc space xy [-1.0 .. 1.0]
fn uv_to_ndc(uv: vec2<f32>) -> vec2<f32> {
    return uv * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
}
/// returns the (0.0, 0.0) .. (1.0, 1.0) position within the viewport for the current render target
/// [0 .. render target viewport size] eg. [(0.0, 0.0) .. (1280.0, 720.0)] to [(0.0, 0.0) .. (1.0, 1.0)]
fn frag_coord_to_uv(frag_coord: vec2<f32>) -> vec2<f32> {
    return (frag_coord - view.viewport.xy) / view.viewport.zw;
}
/// Convert frag coord to ndc
fn frag_coord_to_ndc(frag_coord: vec4<f32>) -> vec3<f32> {
    return vec3(uv_to_ndc(frag_coord_to_uv(frag_coord.xy)), frag_coord.z);
}

fn reconstruct_view_space_position(depth: f32, uv: vec2<f32>) -> vec3<f32> {
    let clip_xy = vec2<f32>(uv.x * 2.0 - 1.0, 1.0 - 2.0 * uv.y);
    let t = view.view_from_clip * vec4<f32>(clip_xy, depth, 1.0);
    let view_xyz = t.xyz / t.w;
    return view_xyz;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let frame = textureSample(screen_texture, texture_sampler, in.uv);

    var depth_buffer_value = textureSample(depth_texture, texture_sampler, in.uv);

    let background = vec3(0.3764706, 0.67254903, 0.99215686);

    if (depth_buffer_value == 0) {
        return vec4(background, 1.0);
    }

    let world_position = position_ndc_to_world(frag_coord_to_ndc(vec4(in.position.xy, depth_buffer_value, 1.0)));

    let distance_depth = length(world_position - view.world_position);

    let chunk_size = 16.0;

    let fog_start = chunk_size*3.0;
    let fog_depth = chunk_size*3.0;

    var visibility = clamp((distance_depth - fog_start) / fog_depth, 0.0, 1.0);

    visibility = pow(visibility, 1.5);

    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        mix(frame.r, background.r, visibility),
        mix(frame.g, background.g, visibility),
        mix(frame.b, background.b, visibility),
        1.0
    );
}
