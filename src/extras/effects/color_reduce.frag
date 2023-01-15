#version 300 es

precision highp float;

in vec2 v_UV;
uniform sampler2D texture0;
uniform float levels;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, v_UV);

    vec4 reduced = round(color * levels) / levels;
    reduced.a = 1.0;

    fragColor = reduced;
}
