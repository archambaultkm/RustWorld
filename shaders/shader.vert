#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture;

out vec2 uv;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    uv = vec2(texture.x, texture.y);
}