#version 300 es

precision highp float;

uniform sampler2D textureSampler;
in vec2 uv;
uniform float time;
out vec4 fragColor;

void main() {
    vec2 shiftUV = uv + vec2(0.0, 0.2 * sin(6.0 * uv.x + time));
    fragColor = texture(textureSampler, shiftUV);
}
