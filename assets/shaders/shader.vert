#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec3 normal;
layout(location=3) in int applied_color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 v_normal;
layout(location=2) out int v_applied_color;

layout(set=1, binding=0) uniform Uniforms {
    mat4 u_view_proj;
};

layout(set=2, binding=0) uniform ModelBindings {
    mat4 model;
};

void main() {
    v_applied_color = applied_color;
    v_normal = normal;
    v_tex_coords = a_tex_coords;


    mat4 modelviewproj = u_view_proj * model;
    gl_Position = modelviewproj * vec4(a_position, 1.0);
}