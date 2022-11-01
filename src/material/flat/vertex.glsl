#version 300 es

const int DIRECTIONAL = 1;
const int POINT = 2;

struct Light {
    int lightType;
    vec4 color;
    vec3 direction;
    vec3 position;
    vec3 attenuation;
};

uniform Light light0;
uniform Light light1;
uniform Light light2;
uniform Light light3;

float lightAttenuation(vec3 attenuation, float distance) {
    return 1.0 / (attenuation[0] + attenuation[1] * distance + attenuation[2] * distance * distance);
}

vec4 lightCalc(Light light, vec3 pointPosition, vec3 pointNormal) {
    float diffuse = 0.0;
    vec3 lightDirection;
    float attenuation = 1.0;
    if (light.lightType == DIRECTIONAL) {
        lightDirection = normalize(light.direction);
    } else if (light.lightType == POINT) {
        lightDirection = normalize(pointPosition - light.position);
        float distance = length(light.position - pointPosition);
        attenuation = lightAttenuation(light.attenuation, distance);
    }
    pointNormal = normalize(pointNormal);
    diffuse = max(dot(pointNormal, - lightDirection), 0.0);
    diffuse *= attenuation;
    return light.color * diffuse;
}

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;

in vec3 vertexPosition;
in vec2 vertexUV;
in vec3 faceNormal;

out vec2 UV;
out vec4 light;

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(vertexPosition, 1.0);
    UV = vertexUV;
    vec3 position = vec3(modelMatrix * vec4(vertexPosition, 1.0));
    vec3 normal = normalize(mat3(modelMatrix) * faceNormal);
    light = vec4(0.0, 0.0, 0.0, 0.0);
    light += lightCalc(light0, position, normal);
    light += lightCalc(light1, position, normal);
    light += lightCalc(light2, position, normal);
    light += lightCalc(light3, position, normal);
}
