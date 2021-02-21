#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec3 Normal;

out VS_OUTPUT {
    vec3 Color;
    vec3 Normal;
    vec3 Position;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
    OUT.Position = Position;
    OUT.Normal = Normal;
}