#version 300 es

precision highp float;

uniform vec4 baseColor;
uniform bool useVertexColors;
in vec4 color;
out vec4 fragColor;

void main() {
    vec4 tempColor = baseColor;

    if (useVertexColors) {
        tempColor *= color;
    }
    fragColor = tempColor;
}
