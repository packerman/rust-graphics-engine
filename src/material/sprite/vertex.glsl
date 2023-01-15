#version 300 es

in vec3 a_position;
in vec2 a_texcoord_0;

uniform mat4 u_ProjectionMatrix;
uniform mat4 u_ViewMatrix;
uniform mat4 u_ModelMatrix;

uniform bool billboard;
uniform float tileNumber;
uniform vec2 tileCount;

out vec2 v_UV;

void main() {
    mat4 mvMatrix = u_ViewMatrix * u_ModelMatrix;
    if (billboard) {
        mvMatrix[0][0] = 1.0;
        mvMatrix[0][1] = 0.0;
        mvMatrix[0][2] = 0.0;
        mvMatrix[1][0] = 0.0;
        mvMatrix[1][1] = 1.0;
        mvMatrix[1][2] = 0.0;
        mvMatrix[2][0] = 0.0;
        mvMatrix[2][1] = 0.0;
        mvMatrix[2][2] = 1.0;
    }
    gl_Position = u_ProjectionMatrix * mvMatrix * vec4(a_position, 1.0);
    v_UV = a_texcoord_0;
    if (tileNumber > -1.0) {
        vec2 tileSize = 1.0 / tileCount;
        float columnIndex = mod(tileNumber, tileCount[0]);
        float rowIndex = floor(tileNumber / tileCount[0]);
        vec2 tileOffset = vec2(columnIndex / tileCount[0], 1.0 - (rowIndex + 1.0)/tileCount[1]);
        v_UV = v_UV * tileSize + tileOffset;
    }
}
