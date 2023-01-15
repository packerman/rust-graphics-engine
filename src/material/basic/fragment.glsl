#version 300 es

precision highp float;

uniform vec4 baseColor;
uniform bool useVertexColors;

in vec4 v_Color;

out vec4 fragColor;

void main() {
    vec4 tempColor = baseColor;

    if (useVertexColors) {
        tempColor *= v_Color;
    }
    fragColor = tempColor;
}
