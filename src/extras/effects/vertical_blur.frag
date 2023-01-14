#version 300 es

precision highp float;

in vec2 v_UV;
uniform sampler2D texture0;
uniform vec2 textureSize;
uniform int blurRadius;
out vec4 fragColor;

void main()
{
    vec2 pixelToTextureCoords = 1.0 / textureSize;
    vec4 averageColor = vec4(0.0, 0.0, 0.0, 0.0);
    for (int offsetY = - blurRadius; offsetY <= blurRadius; offsetY++) {
        float weight = float(blurRadius - abs(offsetY) + 1);
        vec2 offsetUV = vec2(0.0, offsetY) * pixelToTextureCoords;
        averageColor += texture(texture0, v_UV + offsetUV) * weight;
    }
    averageColor /= averageColor.a;
    fragColor = averageColor;
}
