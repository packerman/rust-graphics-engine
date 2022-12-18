#version 300 es

precision highp float;

in vec3 v_Normal;

uniform vec4 u_BaseColorFactor;
uniform vec3 u_Light;
uniform float u_MinFactor;

out vec4 fragColor;

void main() {
    float factor = max(dot(normalize(v_Normal), normalize(-u_Light)), u_MinFactor);
    fragColor = vec4(factor, factor, factor, 1.0) * u_BaseColorFactor;
}
