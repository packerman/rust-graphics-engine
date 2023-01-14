#version 300 es

precision highp float;

in vec2 v_UV;
uniform sampler2D texture0;
uniform vec4 tintColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, v_UV);
    fragColor = vec4(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, 1.0);
}
