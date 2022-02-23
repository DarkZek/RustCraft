// shader.frag
#version 450

// TODO: Create a DISCARD fragment shader to drop work on any pixels that have < 5% opacity, this may also fix my transparency issue

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec3 normal;
layout(location=2) in vec4 v_applied_color;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    //f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords) * max(v_applied_color, vec4(0.05, 0.05, 0.05, 0.05));
    f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords) * v_applied_color;

    //f_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    if (f_color.a < 0.05) {
        discard;
    }
}