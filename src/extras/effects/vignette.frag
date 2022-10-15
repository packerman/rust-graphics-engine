#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform float dimStart;
uniform float dimEnd;
uniform vec4 dimColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, UV);

    vec2 position = 2 * UV - vec2(1.0, 1.0);
    float d = length(position);
    float b = (d - dimEnd) / (dimStart - dimEnd);
    b = clamp(b, 0.0, 1.0);

    fragColor = b * color + (1.0 - b) * dimColor;
}
