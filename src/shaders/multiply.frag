// gaussian.frag
#version 450

layout(location=0) in vec2 TexCoords;

layout(location=0) out vec4 FragColor;

layout(set = 0, binding = 0) uniform texture2D filter_img;
layout(set = 0, binding = 1) uniform texture2D src;
layout(set = 0, binding = 2) uniform sampler samp;

void main() {
    float amount = textureLod(sampler2D(filter_img, samp), TexCoords, 0).r;

    vec3 bloomColor = textureLod(sampler2D(src, samp), TexCoords, 0).rgb;

    FragColor = vec4(bloomColor * amount, 1.0);
}
