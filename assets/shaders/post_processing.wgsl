#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

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

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let frame = textureSample(screen_texture, texture_sampler, in.uv);

    var depth_buffer_value = textureSample(depth_texture, texture_sampler, in.uv);

    let background = vec3(0.3764706, 0.67254903, 0.99215686);

    if (depth_buffer_value == 0) {
        return vec4(background, 1.0);
    }

    let near_plane = 0.1;
    let far_plane = 1000.0;

    // Convert to screen space using close and far pane info
    var ss_depth = near_plane / depth_buffer_value;

    // TODO: Adjust the depth to not be flat to the far plane but take into account the spherical distance from the camera

    let chunk_size = 16.0;

    let fog_start = chunk_size*4.0;
    let fog_depth = chunk_size*2.0;

    let visibility = clamp((ss_depth - fog_start) / fog_depth, 0.0, 1.0);

    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        mix(frame.r, background.r, visibility),
        mix(frame.g, background.g, visibility),
        mix(frame.b, background.b, visibility),
        1.0
    );
}
