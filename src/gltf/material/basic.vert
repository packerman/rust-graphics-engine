#version 300 es

in vec3 a_position;

uniform mat4 u_ModelMatrix;

void main() {
    gl_Position = u_ModelMatrix * vec4(a_position, 1.0);
}
