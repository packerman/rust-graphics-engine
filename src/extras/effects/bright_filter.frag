#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform float threshold;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);
    if (color.r + color.g + color.b < threshold) {
        discard;
    }
    fragColor = color;
}
