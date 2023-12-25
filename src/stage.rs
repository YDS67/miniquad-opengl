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
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        for i in 0..10 {
        for j in 0..10 {
            let xi = i as f32;
            let yj = j as f32;
            vertices.push(Vertex { pos : Vec3 { x: xi+1.0, y: yj+1.0, z: 0.0 }, uv: Vec2 { x: 1., y: 5./5. },
                col: Vec4{x: 1.0, y: 0.0, z: 0.0, w: 1.0} }); // top right
            vertices.push(Vertex { pos : Vec3 { x: xi+1.0, y: yj, z: 0.0 }, uv: Vec2 { x: 1., y: 4./5. },
                col: Vec4{x: 1.0, y: 1.0, z: 0.0, w: 1.0} }); // bottom right
            vertices.push(Vertex { pos : Vec3 { x: xi, y: yj, z: 0.0 }, uv: Vec2 { x: 0., y: 4./5. },
                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // bottom left
            vertices.push(Vertex { pos : Vec3 { x: xi, y: yj+1.0, z: 0.0 }, uv: Vec2 { x: 0., y: 5./5. },
                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // top left 
            vertices.push(Vertex { pos : Vec3 { x: xi + 0.5, y: yj+0.5, z: 0.0 }, uv: Vec2 { x: 0.5, y: 4.5/5. },
                col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // center

            indices.push(5*idx);
            indices.push(5*idx+1);
            indices.push(5*idx+4);
            indices.push(5*idx+1);
            indices.push(5*idx+2);
            indices.push(5*idx+4);
            indices.push(5*idx+2);
            indices.push(5*idx+3);
            indices.push(5*idx+4);
            indices.push(5*idx+3);
            indices.push(5*idx);
            indices.push(5*idx+4);

            idx += 1;
        }}

        for i in 0..10 {
            for j in 5..6 {
                let xi = i as f32;
                let yj = j as f32;
                vertices.push(Vertex { pos : Vec3 { x: xi+1.0, y: yj, z: 1.0 }, uv: Vec2 { x: 1., y: 3./5. },
                    col: Vec4{x: 1.0, y: 0.0, z: 0.0, w: 1.0} }); // top right
                vertices.push(Vertex { pos : Vec3 { x: xi+1.0, y: yj, z: 0.0 }, uv: Vec2 { x: 1., y: 2./5. },
                    col: Vec4{x: 1.0, y: 1.0, z: 0.0, w: 1.0} }); // bottom right
                vertices.push(Vertex { pos : Vec3 { x: xi, y: yj, z: 0.0 }, uv: Vec2 { x: 0., y: 2./5. },
                    col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // bottom left
                vertices.push(Vertex { pos : Vec3 { x: xi, y: yj, z: 1.0 }, uv: Vec2 { x: 0., y: 3./5. },
                    col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // top left 
                vertices.push(Vertex { pos : Vec3 { x: xi + 0.5, y: yj, z: 0.5 }, uv: Vec2 { x: 0.5, y: 2.5/5. },
                    col: Vec4{x: 0.0, y: 1.0, z: 0.0, w: 1.0} }); // center
    
                indices.push(5*idx);
                indices.push(5*idx+1);
                indices.push(5*idx+4);
                indices.push(5*idx+1);
                indices.push(5*idx+2);
                indices.push(5*idx+4);
                indices.push(5*idx+2);
                indices.push(5*idx+3);
                indices.push(5*idx+4);
                indices.push(5*idx+3);
                indices.push(5*idx);
                indices.push(5*idx+4);

                idx += 1;
            }}

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices[..]),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.wall_atlas.clone();
        let dims = pixels.dimensions();
        let texture = ctx.new_texture_from_rgba8(dims.0 as u16, dims.1 as u16, pixels.as_bytes());

        ctx.texture_set_filter(texture, FilterMode::Nearest, MipmapFilterMode::None);

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
