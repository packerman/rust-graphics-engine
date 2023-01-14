#version 300 es

precision highp float;

in vec2 v_UV;
uniform sampler2D texture0;
uniform vec4 tintColor;
out vec4 fragColor;

void main()
{
    vec4 color = texture(texture0, v_UV);
    float gray = (color.r + color.g + color.b) / 3.0;
    fragColor = vec4(gray * tintColor.rgb, 1.0);
}
