#version 330 core
in vec4 cols;
in vec2 uvs;
in vec2 poss;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    if (length(poss) > 0.5) {
        discard;
    } else {
        FragColor = 0.5*(cols+texture(tex, uvs));
    }
}