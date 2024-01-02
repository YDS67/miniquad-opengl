use miniquad::*;

pub const VERTEX_MAIN: &str = include_str!("vertex.glsl");

pub const FRAGMENT_MAIN: &str = include_str!("fragment.glsl");

pub fn meta_main() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("width", UniformType::Float1),
                UniformDesc::new("height", UniformType::Float1),
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsMain {
    pub width: f32,
    pub height: f32,
}
