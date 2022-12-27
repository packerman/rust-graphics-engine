#version 300 es

precision highp float;

in vec3 v_Normal;
in vec2 v_TexCoord_0;

uniform vec4 u_BaseColorFactor;
uniform sampler2D u_BaseColorSampler;
uniform bool u_UseTexture;
uniform bool u_UseLight;
uniform vec3 u_Light;
uniform float u_MinFactor;

out vec4 fragColor;

vec4 getLightFactor() {
    if (u_UseLight) {
        float factor = max(dot(normalize(v_Normal), normalize(-u_Light)), u_MinFactor);
        return vec4(factor, factor, factor, 1.0);
    } else {
        return vec4(1.0, 1.0, 1.0, 1.0);
    }
}

vec4 getTextureColor() {
    if (u_UseTexture) {
        return texture(u_BaseColorSampler, v_TexCoord_0);
    } else {
        return vec4(1.0, 1.0, 1.0, 1.0);
    }
}

void main() {
    fragColor = getLightFactor() * getTextureColor() * u_BaseColorFactor;
}
