#version 300 es

precision highp float;

struct Material {
    vec4 ambient;
    vec4 diffuse;
    bool useTexture;
    sampler2D texture0;
}

uniform Material material;

in vec2 UV;
in vec3 light;

out vec4 fragColor;

void main() {
    vec4 color = vec4(material.diffuse, 1.0);
    if (useTexture) {
        color *= texture(material.texture0, UV);
    }
    color *= vec4(light, 1.0);
    fragColor = material.ambient + color;
}
