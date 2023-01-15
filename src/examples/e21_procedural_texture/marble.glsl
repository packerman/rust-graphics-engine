in vec2 v_UV;
out vec4 fragColor;

void main() {
    float t = fractalRandom(v_UV, 4.0);
    float r = (sin(20.0 * t) + 1.0) / 2.0;
    vec4 color1 = vec4(0.0, 0.2, 0.0, 1.0);
    vec4 color2 = vec4(1.0, 1.0, 1.0, 1.0);
    fragColor = mix(color1, color2, r);
}
