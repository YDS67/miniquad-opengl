#version 330 core
in vec3 pos;
in vec4 col;
in vec2 uv;

uniform float value;

out vec4 cols;
out vec2 uvs;

void main() {
    gl_Position = vec4(pos*value, 1.0);

    cols = col;
    uvs = uv;
}