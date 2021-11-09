#version 450

layout (location = 0) in vec2 position;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in float z_index;
layout (location = 3) in vec4 dimensions;
layout (location = 4) in vec4 uv_dimensions;

layout (location = 0) out vec2 uv;

layout (set = 0, binding = 0) uniform Transform { vec2 transform; };

void main()
{
    uv = vec2(
        mix(uv_dimensions.x, uv_dimensions.x + uv_dimensions.z, tex_coords.x),
        mix(uv_dimensions.y, uv_dimensions.y + uv_dimensions.w, tex_coords.y)
    );

    vec2 rp = vec2(dimensions.x, dimensions.y);
    vec2 rs = vec2(dimensions.z, dimensions.w);

    gl_Position = vec4(vec2(-1.0, 1.0) + transform * vec2(1.0, -1.0) * rp + transform * rs * position, z_index, 1.0);
}
