use image::{self, EncodableLayout, ImageBuffer, Rgba};
use macroquad::prelude::*;
use miniquad::*;

use crate::assets;
use crate::shaders;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
#[repr(C)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec3,
    uv: Vec2,
    col: Vec4,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Stage {
    pub async fn new(ctx: &mut dyn RenderingBackend, ass: &assets::Ass) -> Stage {
        #[rustfmt::skip]
            let quad: [Vertex; 12] = [
                Vertex { pos : Vec3 { x: 5.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 1., y: 3./5. },
                                                                col: Vec4{x: 1.0, y: 0.0, z: 0.0, w: 1.0} }, // top right
                Vertex { pos : Vec3 { x: 5.0, y: 5.0, z: 0.0 }, uv: Vec2 { x: 1., y: 2./5. },
                                                                col: Vec4{x: 1.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom right
                Vertex { pos : Vec3 { x: 4.0, y:5.0, z: 0.0 }, uv: Vec2 { x: 0., y: 2./5. },
                                                                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom left
                Vertex { pos : Vec3 { x: 4.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 0., y: 3./5. } ,
                                                                col: Vec4{x: 0.0, y: 0.0, z: 1.0, w: 1.0} }, // top left 
                Vertex { pos : Vec3 { x: 4.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 1., y: 3./5. },
                                                                col: Vec4{x: 1.0, y: 0.0, z: 0.0, w: 1.0} }, // top right
                Vertex { pos : Vec3 { x: 4.0, y: 5.0, z: 0.0 }, uv: Vec2 { x: 1., y: 2./5. },
                                                                col: Vec4{x: 1.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom right
                Vertex { pos : Vec3 { x: 3.0, y:5.0, z: 0.0 }, uv: Vec2 { x: 0., y: 2./5. },
                                                                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom left
                Vertex { pos : Vec3 { x: 3.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 0., y: 3./5. } ,
                                                                col: Vec4{x: 0.0, y: 0.0, z: 1.0, w: 1.0} }, // top left 
                Vertex { pos : Vec3 { x: 3.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 1., y: 3./5. },
                                                                col: Vec4{x: 1.0, y: 0.0, z: 0.0, w: 1.0} }, // top right
                Vertex { pos : Vec3 { x: 3.0, y: 5.0, z: 0.0 }, uv: Vec2 { x: 1., y: 2./5. },
                                                                col: Vec4{x: 1.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom right
                Vertex { pos : Vec3 { x: 2.0, y:5.0, z: 0.0 }, uv: Vec2 { x: 0., y: 2./5. },
                                                                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }, // bottom left
                Vertex { pos : Vec3 { x: 2.0, y: 5.0, z: 1.0 }, uv: Vec2 { x: 0., y: 3./5. } ,
                                                                col: Vec4{x: 0.0, y: 0.0, z: 1.0, w: 1.0} }, // top left 
            ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&quad),
        );

        let indices: [u16; 18] = [
            0, 1, 3, 1, 2, 3,
            4, 5, 7, 5, 6, 7,
            8, 9, 11, 9, 10, 11,
            ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices[..]),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.wall_atlas.clone();
        let dims = pixels.dimensions();
        let texture = ctx.new_texture_from_rgba8(dims.0 as u16, dims.1 as u16, pixels.as_bytes());

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX,
                    fragment: shaders::FRAGMENT,
                },
                shaders::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("col", VertexFormat::Float4),
            ],
            shader,
        );

        Stage { pipeline, bindings }
    }
}
