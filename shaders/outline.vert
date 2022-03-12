#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 color;

layout(location=0) out vec4 v_color;

layout(set=0, binding=0) uniform Uniforms {
    mat4 u_view;
    mat4 u_proj;
};

void main() {
    v_color = color;

    gl_Position = u_proj * u_view * vec4(a_position, 1.0);
}