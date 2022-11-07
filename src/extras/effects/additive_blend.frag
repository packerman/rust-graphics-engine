#version 300 es

precision highp float;

in vec2 UV;
uniform sampler2D texture0;
uniform sampler2D blendTexture;
uniform float originalStrength;
uniform float blendStrength;
out vec4 fragColor;

void main()
{
    vec4 originalColor = texture(texture0, UV);
    vec4 blendColor = texture(blendTexture, UV);
    fragColor = originalStrength * originalColor + blendStrength * blendColor;
}
