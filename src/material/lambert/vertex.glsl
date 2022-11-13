#version 300 es

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;

in vec3 vertexPosition;
in vec2 vertexUV;
in vec3 vertexNormal;

out vec3 position;
out vec2 UV;
out vec3 normal;

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

vec4 shadowPosition(vec3 position) {
    return shadow0.projectionMatrix * shadow0.viewMatrix * modelMatrix * vec4(position, 1.0);
}

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(vertexPosition, 1.0);
    position = vec3(modelMatrix * vec4(vertexPosition, 1.0));
    UV = vertexUV;
    normal = normalize(mat3(modelMatrix) * vertexNormal);
    if (useShadow) {
        shadowPosition0 = vec3(shadowPosition(vertexPosition));
    }
}
