#version 300 es

precision highp float;

float random(vec2 uv) {
    return fract(235711.0 * sin(14.337 * uv.x + 42.418 * uv.y));
}

float boxRandom(vec2 uv, float scale) {
    vec2 iScaleUV = floor(scale * uv);
    return random(iScaleUV);
}

float smoothRandom(vec2 uv, float scale) {
    vec2 iScaleUV = floor(scale * uv);
    vec2 fScaleUV = fract(scale * uv);
    float a = random(iScaleUV);
    float b = random(round(iScaleUV + vec2(1.0, 0.0)));
    float c = random(round(iScaleUV + vec2(0.0, 1.0)));
    float d = random(round(iScaleUV + vec2(1.0, 1.0)));
    return mix(mix(a, b, fScaleUV.x), mix(c, d, fScaleUV.x), fScaleUV.y);
}

float fractalRandom(vec2 uv, float scale) {
    float value = 0.0;
    float amplitude = 0.5;

    for (int i = 0; i < 6; i++) {
        value += amplitude * smoothRandom(uv, scale);
        scale *= 2.0;
        amplitude *= 0.5;
    }

    return value;
}

in vec2 uv;
out vec4 fragColor;

void main() {
    float r = fractalRandom(uv, 4.0);
    fragColor = vec4(r, r, r, 1);
}
