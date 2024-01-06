use miniquad::*;
use glam::Mat4;

pub const VERTEX: &str = include_str!("vertex.glsl");

pub const FRAGMENT: &str = include_str!("fragment.glsl");

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsMain {
    pub mvp: Mat4,
}
