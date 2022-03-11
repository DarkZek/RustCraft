#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 normal;
layout(location=3) in vec4 applied_color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec4 v_applied_color;
layout(location=3) out vec3 v_position;

layout(set=1, binding=0) uniform Uniforms {
    mat4 u_view;
    mat4 u_proj;
};

layout(set=2, binding=0) uniform ModelBindings {
    mat4 model;
};

void main() {
    v_applied_color = applied_color;
    v_tex_coords = a_tex_coords;

    v_normal = normal;

    vec4 viewPos = u_view * model * vec4(a_position, 1.0);

    v_position = viewPos.xyz;

    gl_Position = u_proj * viewPos;
}