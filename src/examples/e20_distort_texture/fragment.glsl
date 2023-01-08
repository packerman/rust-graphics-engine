#version 300 es

precision highp float;

uniform sampler2D noise;
uniform sampler2D image;
in vec2 v_UV;
uniform float time;
out vec4 fragColor;

void main() {
    vec2 uvShift = v_UV + vec2(-0.033, 0.07) * time;
    vec4 noiseValues = texture(noise, uvShift);
    vec2 uvNoise = v_UV + 0.4*noiseValues.rg;
    fragColor = texture(image, uvNoise);
}
