// gaussian.frag
#version 450

layout(location=0) in vec2 TexCoords;

layout(location=0) out vec4 FragColor;

layout(set = 0, binding = 0) uniform texture2D src;
layout(set = 0, binding = 1) uniform texture2D dest;
layout(set = 0, binding = 2) uniform sampler samp;

void main() {
    const float gamma = 2.2;
    const float exposure = 1.0;

    vec3 hdrColor = textureLod(sampler2D(dest, samp), TexCoords, 0).rgb;

    vec3 bloomColor = textureLod(sampler2D(src, samp), TexCoords, 0).rgb;
    hdrColor += bloomColor; // additive blending
    // tone mapping
    vec3 result = vec3(1.0) - exp(-hdrColor * exposure);

    // also gamma correct while we're at it
    //result = pow(result, vec3(1.0 / gamma));

    FragColor = vec4(result, 1.0);
}
