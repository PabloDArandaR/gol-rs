#version 420 core
uniform vec4 aliveColor;
out vec4 FragColor;

void main()
{
	FragColor = aliveColor;
}
