#version 300 es

in vec2 a_position;
in vec2 a_texcoord_0;
out vec2 v_UV;

void main()
{
    gl_Position = vec4(a_position, 0.0, 1.0);
    v_UV = a_texcoord_0;
}
