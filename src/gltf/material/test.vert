#version 300 es

in vec3 a_position;
in vec3 a_normal;

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;
uniform mat4 u_NormalMatrix;

out vec3 v_Normal;

void main() {
    gl_Position = u_ViewProjectionMatrix * u_ModelMatrix * vec4(a_position, 1.0);
    v_Normal = vec3(u_NormalMatrix * vec4(a_normal, 0.0));
}
