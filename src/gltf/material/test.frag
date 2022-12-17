#version 300 es

precision highp float;

in vec3 v_Normal;
in vec3 v_Light;

uniform vec4 u_BaseColorFactor;

out vec4 fragColor;

void main() {
    float factor = max(dot(normalize(v_Normal), v_Light), 0.1);
    fragColor = vec4(factor, factor, factor, 1.0) * u_BaseColorFactor;
    fragColor = u_BaseColorFactor;
}
