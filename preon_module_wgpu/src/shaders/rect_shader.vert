#version 450

layout (location = 0) in vec2 position;
layout (location = 1) in vec4 radius;
layout (location = 2) in vec4 dimensions;
layout (location = 3) in vec4 color;

layout (location = 0) out float ratioOut;
layout (location = 1) out vec4 radiusOut;
layout (location = 2) out vec4 colorOut;

layout (set = 0, binding = 0) uniform Transform { vec2 transform; };

void main()
{
    ratioOut = dimensions.z / dimensions.w;
    radiusOut = radius;
    colorOut = color;

    vec2 rp = vec2(dimensions.x, dimensions.y);
    vec2 rs = vec2(dimensions.z, dimensions.w);

    gl_Position = vec4(vec2(-1.0, 1.0) + transform * vec2(1.0, -1.0) * rp + transform * rs * position, 0.0, 1.0);
}