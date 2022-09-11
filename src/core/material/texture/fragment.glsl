#version 300 es

precision highp float;

uniform vec4 baseColor;
uniform sampler2D texture;
in vec2 uv;
out vec4 fragColor;

void main() {
    vec4 color = baseColor * texture2D(texture, uv);
    if (color.a < 0.10) {
        discard;
    }
    fragColor = color;
}
