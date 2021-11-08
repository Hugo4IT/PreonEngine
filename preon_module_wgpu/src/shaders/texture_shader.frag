#version 450

layout (location = 0) in vec2 uv;

layout (set = 1, binding = 0) uniform texture2D inTexture;
layout (set = 1, binding = 1) uniform sampler inSampler;

layout (location = 0) out vec4 FragColor;

void main()
{
    FragColor = texture(sampler2D(inTexture, inSampler), uv);
}
