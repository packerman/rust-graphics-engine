#version 300 es

in vec3 a_position;
in vec3 a_normal;

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;

out vec3 v_Normal;
out vec3 v_Light;

void main() {
    gl_Position = u_ViewProjectionMatrix * u_ModelMatrix * vec4(a_position, 1.0);
    v_Normal = a_normal;
    v_Light = normalize(vec3(-1.0, -1.0, -1.0));
}
