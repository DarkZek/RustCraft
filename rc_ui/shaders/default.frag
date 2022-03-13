// shader.frag
#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec4 v_color;

layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;

void main() {
    f_color = v_color;

    if (v_tex_coords.x != -1) {
        // Rendering image
        vec4 texture_color = textureLod(sampler2D(t_diffuse, s_diffuse), v_tex_coords, 0);

        f_color = texture_color * v_color;
    }

    if (f_color.a == 0) {
        discard;
    }
}