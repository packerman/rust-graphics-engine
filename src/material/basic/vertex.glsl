#version 300 es

in vec3 a_position;
in vec4 a_color_0;

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;
uniform float pointSize;

out vec4 v_Color;

void main() {
    gl_PointSize = pointSize;
    gl_Position = u_ViewProjectionMatrix * u_ModelMatrix * vec4(a_position, 1.0);
    v_Color = a_color_0;
}
