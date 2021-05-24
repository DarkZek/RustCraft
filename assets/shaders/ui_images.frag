// shader.frag
#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) in vec4 v_color;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D images_array_diffuse[];
layout(set = 0, binding = 1) uniform sampler images_sampler_diffuse;

void main() {
    f_color = vec4(1.0, 1.0, 1.0, 0.0);
}