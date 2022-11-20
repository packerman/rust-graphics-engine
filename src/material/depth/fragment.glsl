#version 300 es

precision highp float;

out vec4 fragColor;

void main() {
    float z = gl_FragCoord.z;
    fragColor = vec4(z, z, z, 1.0);
}
