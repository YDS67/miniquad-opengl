use miniquad::*;

pub const VERTEX: &str = r#"#version 330
attribute vec3 pos;
attribute vec2 uv;
attribute vec4 col;

uniform vec3 playerpos;
uniform vec2 playerdir;

varying lowp vec2 texcoord;
varying lowp vec4 cols;

float pi, fov, asp, dxy, at, phi, u, d, bt, theta, v;

float angle_to_range(float pi, float a) {
    while (a < -pi/2.0) {
        a += 2.0*pi;
    };
    while (a > pi/2.0) {
        a -= 2.0*pi;
    };
    return a;
};

void main() {
    pi = 3.1415926538;
    fov = pi/2.0;
    asp = 800.0/1280.0;

    dxy = distance(playerpos.xy, pos.xy);
    at = sign(pos.y-playerpos.y) * acos((pos.x-playerpos.x)/dxy);
    phi = angle_to_range(pi, fov/2.0 + playerdir.x - at);
    u = phi/fov - 0.5;

    d = distance(playerpos, pos);
    bt = pi/2.0 - acos((pos.z-playerpos.z)/d);
    theta = angle_to_range(pi, asp * fov/2.0 + playerdir.y - bt);
    v = 0.5 - theta/fov/asp;

    gl_Position = vec4(u, v, 0, 1);
    texcoord = uv;
    cols = col;
}"#;

pub const FRAGMENT: &str = r#"#version 330
varying lowp vec2 texcoord;
varying lowp vec4 cols;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture(tex, texcoord);
}"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("playerpos", UniformType::Float3), UniformDesc::new("playerdir", UniformType::Float2)],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub playerpos: (f32, f32, f32),
    pub playerdir: (f32, f32),
}
