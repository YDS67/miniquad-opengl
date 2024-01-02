#version 330 core
in vec4 cols;
in vec2 uvs;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    FragColor = 0.5*(cols+texture(tex, uvs));
}