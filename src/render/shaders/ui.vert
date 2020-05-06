#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

layout(set=1, binding=0) uniform Uniforms {
    mat4 view_proj;
};

void main() {
    v_tex_coords = a_tex_coords;

    gl_Position = vec4(a_position, 0.0, 1.0) * view_proj;
}