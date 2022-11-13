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
};

uniform Light light0;
uniform Light light1;
uniform Light light2;
uniform Light light3;

struct Material {
    vec4 ambient;
    vec4 diffuse;
    bool useTexture;
    sampler2D texture0;
    bool useBumpTexture;
    sampler2D bumpTexture;
    float bumpStrength;
};

uniform Material material;

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
in vec3 shadowPosition0;

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

in vec3 position;
in vec2 UV;
in vec3 normal;

bool fragmentInShadow() {
    if (dot(normalize(normal), -normalize(shadow0.lightDirection)) <= 0.01) {
        return false;
    }
    vec3 shadowCoord = (shadowPosition0 + 1.0) / 2.0;
    float closestDistanceToLight = texture(shadow0.depthTexture, shadowCoord.xy).r;
    float fragmentDistanceToLight = clamp(shadowCoord.z, 0.0, 1.0);
    return fragmentDistanceToLight > closestDistanceToLight + shadow0.bias;
}

out vec4 fragColor;

void main() {
    vec4 color = material.diffuse;
    if (material.useTexture) {
        color *= texture(material.texture0, UV);
    }

    vec3 bumpNormal = normal;
    if (material.useBumpTexture) {
        bumpNormal += material.bumpStrength * vec3(texture(material.bumpTexture, UV));
    }

    vec4 total = vec4(0.0, 0.0, 0.0, 0.0);
    total += lightCalc(light0, position, bumpNormal);
    total += lightCalc(light1, position, bumpNormal);
    total += lightCalc(light2, position, bumpNormal);
    total += lightCalc(light3, position, bumpNormal);
    color *= vec4(total.xyz, 1.0);
    color += material.ambient;
    if (useShadow && fragmentInShadow()) {
        float s = 1.0 - shadow0.strength;
        color *= vec4(s, s, s, 1.0);
    }
    fragColor = color;
}
