#version 300 es

precision highp float;

const int OPAQUE_ALPHA_MODE = 0;
const int MASK_ALPHA_MODE = 1;
const int BLEND_ALPHA_MODE = 2;

in vec3 v_Normal;
in vec2 v_TexCoord_0;
in vec4 v_Color_0;

uniform vec4 u_BaseColorFactor;
uniform sampler2D u_BaseColorSampler;
uniform bool u_UseTexture;
uniform bool u_UseLight;
uniform vec3 u_Light;
uniform float u_MinFactor;
uniform bool u_UseColor_0;

uniform int u_AlphaMode;
uniform float u_AlphaCutoff;

out vec4 FragColor;

vec4 getLightFactor() {
    if (u_UseLight) {
        float factor = max(dot(normalize(v_Normal), normalize(-u_Light)), u_MinFactor);
        return vec4(factor, factor, factor, 1.0);
    } else {
        return vec4(1.0);
    }
}

vec4 getTextureColor() {
    if (u_UseTexture) {
        return texture(u_BaseColorSampler, v_TexCoord_0);
    } else {
        return vec4(1.0);
    }
}

vec4 getVertexColor() {
    if (u_UseColor_0) {
        return v_Color_0;
    } else {
        return vec4(1.0);
    }
}

void main() {
    vec4 baseColor = getLightFactor() * 
                        getTextureColor() * 
                        getVertexColor() * u_BaseColorFactor;
    if (u_AlphaMode == OPAQUE_ALPHA_MODE) {
        baseColor.a = 1.0;
    } else if (u_AlphaMode == MASK_ALPHA_MODE) {
        if (baseColor.a < u_AlphaCutoff) {
            discard;
        }
        baseColor.a = 1.0;
    }
    FragColor = baseColor;
}
