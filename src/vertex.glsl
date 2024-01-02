#version 330 core
in vec3 pos;
in vec4 col;
in vec2 uv;

uniform float width;
uniform float height;

out vec4 cols;
out vec2 uvs;

void main() {
    gl_Position = vec4(pos*vec3(width, height, 0.0), 1.0);

    cols = col;
    uvs = uv;
}