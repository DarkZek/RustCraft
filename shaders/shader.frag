// shader.frag
#version 450

// TODO: Create a DISCARD fragment shader to drop work on any pixels that have < 5% opacity, this may also fix my transparency issue

layout(location=0) in vec2 texture_coords;
layout(location=1) in vec3 normal;
layout(location=2) in vec4 applied_color;
layout(location=3) in vec4 lighting_color;
layout(location=4) in vec3 position;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 bloom_color;
layout(location=2) out vec4 normal_color;
layout(location=3) out vec4 position_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {

    vec4 ambient = vec4(0.05, 0.05, 0.05, 1.0);
    vec4 lighting_range = vec4(0.7, 0.7, 0.7, 1.0);

    // Mix the lighting affect from black to the color depending on how bright the light is
    vec4 lighting = mix(vec4(0.0, 0.0, 0.0, 1.0), vec4(lighting_color.rgb, 1.0), lighting_color.a);

    // Take the ambient light and add the lighting range
    vec4 lighting_multiplier = ambient + (lighting_range * lighting);

    f_color = texture(sampler2D(t_diffuse, s_diffuse), texture_coords) * applied_color * lighting_multiplier;

    if (f_color.a < 0.05) {
        discard;
    }

    // https://learnopengl.com/Advanced-Lighting/Bloom
    float brightness = dot(f_color.rgb, vec3(0.2126, 0.7152, 0.0722));

    if(brightness > 0.992) {
        bloom_color = vec4(f_color.rgb, 1.0);
    } else {
        bloom_color = vec4(0,0,0, 1.0);
    }

    normal_color = vec4(normalize(normal), 1.0);
    position_color = vec4(position, 1.0);
}