#version 330 core
in vec3 pos;
in vec4 col;
in vec2 uv;

uniform mat4 mvp;

out vec4 cols;
out vec2 uvs;
out vec2 poss;

void main() {
    poss = pos.xy;
    gl_Position = mvp * vec4(pos, 1.0);

    cols = col;
    uvs = uv;
}