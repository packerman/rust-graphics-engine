#version 300 es

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;

in vec3 a_position;
in vec2 a_texcoord_0;
in vec3 a_normal;

out vec3 v_Position;
out vec2 v_UV;
out vec3 v_Normal;

struct Shadow {
    vec3 lightDirection;
    mat4 projectionMatrix;
    mat4 viewMatrix;
    sampler2D depthTexture;
    float strength;
    float bias;
};

uniform bool useShadow;
uniform Shadow shadow0;
out vec3 shadowPosition0;

vec4 shadowPosition(vec4 worldPosition) {
    return shadow0.projectionMatrix * shadow0.viewMatrix * worldPosition;
}

void main() {
    vec4 worldPosition = u_ModelMatrix * vec4(a_position, 1.0);
    gl_Position = u_ViewProjectionMatrix * worldPosition;
    v_Position = vec3(worldPosition);
    v_UV = a_texcoord_0;
    v_Normal = normalize(mat3(u_ModelMatrix) * a_normal);
    if (useShadow) {
        shadowPosition0 = vec3(shadowPosition(worldPosition));
    }
}
