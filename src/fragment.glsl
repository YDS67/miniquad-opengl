#version 330 core
in vec4 v_col;
in vec2 v_uv;
in vec2 v_pos;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    if (v_pos.x*v_pos.x*v_pos.x*v_pos.x+v_pos.y*v_pos.y*v_pos.y*v_pos.y > 0.5*0.5*0.5*0.5) {
        discard;
    } else {
        FragColor = 0.5*(v_col+texture(tex, v_uv));
    }
}