#version 300 es

precision highp float;

uniform sampler2D textureSampler1;
uniform sampler2D textureSampler2;
in vec2 uv;
uniform float time;
out vec4 fragColor;

void main() {
    vec4 color1 = texture(textureSampler1, uv);
    vec4 color2 = texture(textureSampler2, uv);
    float s = abs(sin(time));
    fragColor = s * color1 + (1.0 - s) * color2;
}
