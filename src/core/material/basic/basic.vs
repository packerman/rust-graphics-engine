#version 300 es

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;
uniform float pointSize;
in vec3 vertexPosition;
in vec4 vertexColor;
out vec4 color;

void main() {
    gl_PointSize = pointSize;
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(vertexPosition, 1.0);
    color = vertexColor;
}
