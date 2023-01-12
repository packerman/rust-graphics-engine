in vec2 v_UV;
out vec4 fragColor;

void main() {
    float r = fractalRandom(v_UV, 5.0);
    vec4 color1 = vec4(0.5, 0.5, 1.0, 1.0);
    vec4 color2 = vec4(1.0, 1.0, 1.0, 1.0);
    fragColor = mix(color1, color2, r);
}
