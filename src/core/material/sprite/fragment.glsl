#version 300 es

precision highp float;

uniform vec4 baseColor;
uniform sampler2D texture0;
in vec2 uv;
out vec4 fragColor;

void main() {
    vec4 color = baseColor * texture(texture0, uv);
    if (color.a < 0.1) {
        discard;
    }
    fragColor = color;
}
