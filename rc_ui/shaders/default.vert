#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec4 v_color;

layout(set=0, binding=0) uniform Uniforms {
    vec2 viewport_size;
};

void main() {
    v_tex_coords = a_tex_coords;
    v_color = color;

    // Convert from 0-size to 0-size*2 since it needs to come out to -1 to 1, not 0 to 1
    vec2 pos = a_position * 2;

    gl_Position = vec4((pos.x / viewport_size.x) - 1, -(pos.y / viewport_size.y) + 1, 1.0, 1.0);
}