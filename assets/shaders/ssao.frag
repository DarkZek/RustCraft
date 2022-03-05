// gaussian.frag
#version 450

layout(location=0) in vec2 TexCoords;

layout(location=0) out vec4 FragColor;

layout(set = 0, binding = 0) uniform texture2D gPosition;
layout(set = 0, binding = 1) uniform texture2D gNormal;
layout(set = 0, binding = 2) uniform texture2D texNoise;
layout(set = 0, binding = 3) uniform sampler samp;

layout(set = 0, binding = 4) uniform KernelSamples {
    vec3 samples[64];
};

int kernelSize = 64;
float radius = 0.5;
float bias = 0.025;

// tile noise texture over screen based on screen dimensions divided by noise size
const vec2 noiseScale = vec2(800.0/4.0, 600.0/4.0);

layout(set=1, binding=0) uniform Uniforms {
    mat4 u_view;
    mat4 projection;
};

void main() {
    vec3 fragPos = texture(sampler2D(gPosition, samp), TexCoords).xyz;

    vec3 normal = normalize(texture(sampler2D(gNormal, samp), TexCoords).rgb);
    vec3 randomVec = normalize(texture(sampler2D(texNoise, samp), TexCoords * noiseScale).xyz);

    // create TBN change-of-basis matrix: from tangent-space to view-space
    vec3 tangent = normalize(randomVec - normal * dot(randomVec, normal));
    vec3 bitangent = cross(normal, tangent);
    mat3 TBN = mat3(tangent, bitangent, normal);
    // iterate over the sample kernel and calculate occlusion factor
    float occlusion = 0.0;

    for(int i = 0; i < kernelSize; ++i)
    {
        // get sample position
        vec3 samplePos = TBN * samples[i]; // from tange/nt to view-space
        samplePos = fragPos + samplePos * radius;

        // project sample position (to sample texture) (to get position on screen/texture)
        vec4 offset = vec4(samplePos, 1.0);
        offset = projection * offset; // from view to clip-space
        offset.xyz /= offset.w; // perspective divide
        offset.xyz = offset.xyz * 0.5 + 0.5; // transform to range 0.0 - 1.0

        // get sample depth
        float sampleDepth = textureLod(sampler2D(gPosition, samp), offset.xy, 0).z; // get depth value of kernel sample

        // range check & accumulate
        float rangeCheck = smoothstep(0.0, 1.0, radius / abs(fragPos.z - sampleDepth));
        occlusion += (sampleDepth >= samplePos.z + bias ? 1.0 : 0.0) * rangeCheck;
    }

    occlusion = 1.0 - (occlusion / kernelSize);

    FragColor = vec4(occlusion, occlusion, occlusion, 1.0);
}
