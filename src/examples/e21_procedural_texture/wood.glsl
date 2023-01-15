in vec2 v_UV;
out vec4 fragColor;

void main() {
    float t = 80.0 * v_UV.y + 20.0 * fractalRandom(v_UV, 2.0);
    float r = clamp(sin(t) + 1.0, 0.0, 1.0);
    vec4 color1 = vec4(0.3, 0.2, 0.0, 1.0);
    vec4 color2 = vec4(0.6, 0.4, 0.2, 1.0);
    fragColor = mix(color1, color2, r);
}
