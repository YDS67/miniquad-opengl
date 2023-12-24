use miniquad::*;

pub const VERTEX: &str = r#"#version 100
attribute vec3 pos;
attribute vec2 uv;
attribute vec4 col;

uniform vec2 offset;

varying lowp vec2 texcoord;
varying lowp vec4 cols;

void main() {
    gl_Position = vec4(pos.xy + offset, pos.z, 1);
    texcoord = uv;
    cols = col;
}"#;

pub const FRAGMENT: &str = r#"#version 100
varying lowp vec4 cols;

uniform sampler2D tex;

void main() {
    gl_FragColor = cols;
}"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub offset: (f32, f32),
}
