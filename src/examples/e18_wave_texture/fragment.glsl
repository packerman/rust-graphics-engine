#version 300 es

precision highp float;

uniform sampler2D textureSampler;
in vec2 v_UV;
uniform float time;
out vec4 fragColor;

void main() {
    vec2 shiftUV = v_UV + vec2(0.0, 0.2 * sin(6.0 * v_UV.x + time));
    fragColor = texture(textureSampler, shiftUV);
}
