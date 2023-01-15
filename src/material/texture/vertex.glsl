#version 300 es

in vec3 a_position;
in vec2 a_texcoord_0;

uniform mat4 u_ModelMatrix;
uniform mat4 u_ViewProjectionMatrix;

uniform vec2 repeatUV;
uniform vec2 offsetUV;

out vec2 uv;

void main() {
    gl_Position = u_ViewProjectionMatrix * u_ModelMatrix * vec4(a_position, 1.0);
    uv = a_texcoord_0 * repeatUV + offsetUV;
}
