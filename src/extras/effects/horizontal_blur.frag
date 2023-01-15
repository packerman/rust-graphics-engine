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
    for (int offsetX = - blurRadius; offsetX <= blurRadius; offsetX++) {
        float weight = float(blurRadius - abs(offsetX) + 1);
        vec2 offsetUV = vec2(offsetX, 0.0) * pixelToTextureCoords;
        averageColor += texture(texture0, v_UV + offsetUV) * weight;
    }
    averageColor /= averageColor.a;
    fragColor = averageColor;
}
