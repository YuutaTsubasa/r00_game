#version 300 es
precision mediump float;

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;
layout(location = 2) in vec2 aTexCoord;

uniform mat4 uProjection;

out vec4 ourColor;
out vec2 TexCoord;

void main()
{
    gl_Position = uProjection * vec4(aPos, 1.0);
    ourColor = aColor;
    TexCoord = aTexCoord;
}