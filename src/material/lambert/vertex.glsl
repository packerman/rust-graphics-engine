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

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(vertexPosition, 1.0);
    vec3 position = vec3(modelMatrix * vec4(vertexPosition, 1.0));
    UV = vertexUV;
    vec3 normal = normalize(mat3(modelMatrix) * vertexNormal);
}
