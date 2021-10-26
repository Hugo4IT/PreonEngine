#version 450

layout (location = 0) in float ratio;
layout (location = 1) in vec4 radius;
layout (location = 2) in vec4 color;

layout (location = 0) out vec4 FragColor;

void main()
{
    FragColor = color;
}