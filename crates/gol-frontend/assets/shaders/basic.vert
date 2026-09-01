#version 420 core
layout (location = 0) in vec2 meshPos;
layout (location = 1) in ivec2 cellPos;

void main()
{
    gl_Position = vec4(cellPos.x + meshPos.x, cellPos.y + meshPos.y, 1.0, 1.0);
}
