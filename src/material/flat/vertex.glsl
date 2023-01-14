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
    if (light.lightType > 0) {
        pointNormal = normalize(pointNormal);
        diffuse = max(dot(pointNormal, - lightDirection), 0.0);
        diffuse *= attenuation;
    }
    return light.color * diffuse;
}

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;

in vec3 a_position;
in vec2 a_texcoord_0;
in vec3 a_normal;

out vec2 v_UV;
out vec4 v_Light;

void main() {
    vec4 worldPosition = u_ModelMatrix * vec4(a_position, 1.0);
    gl_Position = u_ViewProjectionMatrix * worldPosition;
    v_UV = a_texcoord_0;
    vec3 position = vec3(worldPosition);
    vec3 normal = normalize(mat3(u_ModelMatrix) * a_normal);
    v_Light = vec4(0.0, 0.0, 0.0, 0.0);
    v_Light += lightCalc(light0, position, normal);
    v_Light += lightCalc(light1, position, normal);
    v_Light += lightCalc(light2, position, normal);
    v_Light += lightCalc(light3, position, normal);
}
