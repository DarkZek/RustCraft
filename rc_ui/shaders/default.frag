// shader.frag
#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec4 v_color;

layout(location=0) out vec4 f_color;

//layout(set = 0, binding = 0) uniform texture2D t_diffuse;
//layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    // Required or else background of text randomly stops appearing
//    vec4 texture_color = v_tex_coords == vec2(0.0, 0.0) ? vec4(1.0, 1.0, 1.0, 0.0) : textureLod(sampler2D(t_diffuse, s_diffuse), v_tex_coords, 0);
//
//    f_color = v_tex_coords == vec2(-1.0, -1.0) ? v_color : texture_color;
    f_color = v_color;
}