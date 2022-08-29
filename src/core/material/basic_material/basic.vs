#version 300 es

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;
in vec4 vertexPosition;
int vec4 vertexColor;
out vec4 color;

void main() {
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vertexPosition;
    color = vertexColor;
}
