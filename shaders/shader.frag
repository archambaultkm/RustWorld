#version 330 core

out vec4 FragColour;

in vec2 texture_coordinate;

//texture sampler
uniform sampler2D texture1;

void main() {
    FragColour = texture(texture1, texture_coordinate); // 0.2 returns 80% first input colour, 20% the second.
}