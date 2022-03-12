// gaussian.frag
#version 450

layout(location=0) out vec4 FragColor;

layout(location=0) in vec2 TexCoords;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(push_constant) uniform PushConstants {
    int horizontal;
};

void main() {

    float weight[5] = float[] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

    vec2 tex_offset = 1.0 / textureSize(sampler2D(t_diffuse, s_diffuse), 0); // gets size of single texel
    vec3 result = texture(sampler2D(t_diffuse, s_diffuse), TexCoords).rgb * weight[0]; // current fragment's contribution

    if(horizontal == 1)
    {
        for(int i = 1; i < 5; ++i)
        {
            result += textureLod(sampler2D(t_diffuse, s_diffuse), TexCoords + vec2(tex_offset.x * i, 0.0), 0).rgb * weight[i];
            result += textureLod(sampler2D(t_diffuse, s_diffuse), TexCoords - vec2(tex_offset.x * i, 0.0), 0).rgb * weight[i];
        }
    }
    else
    {
        for(int i = 1; i < 5; ++i)
        {
            result += textureLod(sampler2D(t_diffuse, s_diffuse), TexCoords + vec2(0.0, tex_offset.y * i), 0).rgb * weight[i];
            result += textureLod(sampler2D(t_diffuse, s_diffuse), TexCoords - vec2(0.0, tex_offset.y * i), 0).rgb * weight[i];
        }
    }

    FragColor = vec4(result, 1.0);
}
