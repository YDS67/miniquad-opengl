use miniquad::*;

pub const VERTEX_MAIN: &str = include_str!("vertex.glsl");

pub const FRAGMENT_MAIN: &str = include_str!("fragment.glsl");

pub fn meta_main() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("value", UniformType::Float1),
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsMain {
    pub value: f32,
}
