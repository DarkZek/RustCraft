#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 color;

layout(location=0) out vec2 v_tex_coord;

void main() {
    v_tex_coord = a_tex_coords;

    gl_Position = vec4(a_position, 0.0, 1.0);
}