#version 330 core
in vec3 pos;
in vec4 col;
in vec2 uv;

uniform mat4 mvp;

out vec4 v_col;
out vec2 v_uv;
out vec2 v_pos;

void main() {
    v_pos = pos.xy;
    gl_Position = mvp * vec4(pos, 1.0);

    v_col = col;
    v_uv = uv;
}