#version 420 core
layout (location = 0) in vec2 meshPos;
layout (location = 1) in ivec2 cellPos;

void main()
{
	vec2 pos = meshPos + vec2(cellPos);
    	gl_Position = vec4(pos, 0.0, 1.0);
}
