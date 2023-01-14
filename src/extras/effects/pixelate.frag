#version 300 es

precision highp float;

in vec2 v_UV;
uniform sampler2D texture0;
uniform float pixelSize;
uniform vec2 resolution;
out vec4 fragColor;

void main()
{
    vec2 factor = resolution / pixelSize;
    vec2 newUV = floor(v_UV * factor) / factor;
    fragColor = texture(texture0, newUV);
}
