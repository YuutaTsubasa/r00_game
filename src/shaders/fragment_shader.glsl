#version 300 es
precision mediump float;

in vec4 ourColor;
in vec2 TexCoord;

out vec4 FragColor;

uniform sampler2D mainTexture;

void main()
{
    vec4 texColor = texture(mainTexture, TexCoord);
    FragColor = texColor * ourColor;
}