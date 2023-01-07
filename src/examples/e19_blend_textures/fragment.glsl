#version 300 es

precision highp float;

uniform sampler2D textureSampler1;
uniform sampler2D textureSampler2;
in vec2 v_UV;
uniform float time;
out vec4 fragColor;

void main() {
    vec4 color1 = texture(textureSampler1, v_UV);
    vec4 color2 = texture(textureSampler2, v_UV);
    float s = (sin(time) + 1.0) / 2.0;
    fragColor = s * color1 + (1.0 - s) * color2;
}
