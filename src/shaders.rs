use miniquad::*;

pub const VERTEX: &str = r#"#version 330
attribute vec3 pos;
attribute vec2 uv;
attribute vec4 col;

uniform vec3 playerpos;
uniform vec4 playerdir;

varying vec3 texcoord;
varying vec4 cols;

const float pi = 3.1415926538;
const float fov = pi/4.0;
const float asp = 800.0/1280.0;

float at, u, bt, v, d;

vec3 dir1, dir2, cros, nor1, nor2;

void main() {

    dir1 = vec3(playerdir.xy*playerdir.w, playerdir.z);
    dir2 = pos - playerpos;
    nor1 = vec3(0.0,0.0,1.0);
    nor2 = vec3(vec2(playerdir.y,-playerdir.x),0);

    cros = cross(dir1, dir2);
    d = dot(dir1, dir2);

    at = atan(-dot(cros, nor1),d);
    bt = atan(dot(cros, nor2),d);

    u = 2.0*sin(at)/sin(fov);
    v = 2.0*sin(bt)/sin(fov*asp);

    gl_Position = vec4(u, v, 0, 1);
    texcoord = vec3(uv/length(dir2),1.0/length(dir2));

    if (at < -1.4*fov || at > 1.4*fov || bt < -1.4*fov*asp || bt > 1.4*fov*asp) {
        cols = vec4(1.0,1.0,1.0,0.0);
    } else {
        cols = col;
    };
    
}"#;

pub const FRAGMENT: &str = r#"#version 330
varying vec3 texcoord;
varying vec4 cols;

uniform sampler2D tex;

void main() {
    if (cols.w < 1.0) {
        discard;
    } else {
        gl_FragColor = textureProj(tex, texcoord);
    }
}"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("playerpos", UniformType::Float3), UniformDesc::new("playerdir", UniformType::Float4)],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub playerpos: (f32, f32, f32),
    pub playerdir: (f32, f32, f32, f32),
}
