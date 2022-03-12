#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec4 v_color;

layout( push_constant ) uniform constants
{
    float xOffset;
    float yOffset;
} PushConstants;

void main() {
    v_tex_coords = a_tex_coords;
    v_color = color;

    gl_Position = vec4(a_position.x + PushConstants.xOffset, a_position.y + PushConstants.yOffset, 0.0, 1.0);
}