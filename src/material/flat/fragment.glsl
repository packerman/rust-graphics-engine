#version 300 es

precision highp float;

struct Material {
    vec4 ambient;
    vec4 diffuse;
    bool useTexture;
    sampler2D texture0;
};

uniform Material material;

in vec2 UV;
in vec4 light;

out vec4 fragColor;

void main() {
    vec4 color = material.diffuse;
    if (material.useTexture) {
        color *= texture(material.texture0, UV);
    }
    color *= vec4(light.xyz, 1.0);
    fragColor = material.ambient + color;
}
