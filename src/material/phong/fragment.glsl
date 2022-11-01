#version 300 es

precision highp float;

const int DIRECTIONAL = 1;
const int POINT = 2;

struct Light {
    int lightType;
    vec4 color;
    vec3 direction;
    vec3 position;
    vec3 attenuation;
}

uniform Light light0;
uniform Light light1;
uniform Light light2;
uniform Light light3;

struct Material {
    vec4 ambient;
    vec4 diffuse;
    bool useTexture;
    sampler2D texture0;
    float specularStrength;
    float shininess;
}

uniform Material material;

uniform vec3 viewPosition;

float lightAttenuation(vec3 attenuation, float distance) {
    return 1.0 / (attenuation[0] + attenuation[1] * distance + attenuation[2] * distance * distance);
}

vec3 lightCalc(Light light, vec3 pointPosition, vec3 pointNormal) {
    float diffuse = 0.0;
    float specular = 0.0;
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
    if (diffuse > 0.0) {
        vec3 viewDirection = normalize(viewPosition - pointPosition);
        vec3 reflectDirection = reflect(lightDirection, pointNormal);
        specular = max(dot(viewDirection, reflectDirection), 0.0);
        specular = specularStrength * pow(specular, shininess);
    }
    diffuse *= attenuation;
    return light.color * (diffuse + specular);
}

in vec3 position;
in vec2 UV;
in vec3 normal;

out vec4 fragColor;

void main() {
    vec4 color = vec4(material.diffuse, 1.0);
    if (useTexture) {
        color *= texture(material.texture0, UV);
    }
    vec3 total = vec3(0.0, 0.0, 0.0);
    total += lightCalc(light0, position, normal);
    total += lightCalc(light1, position, normal);
    total += lightCalc(light2, position, normal);
    total += lightCalc(light3, position, normal);
    color *= vec4(total, 1.0);
    fragColor = material.ambient + color;
}
