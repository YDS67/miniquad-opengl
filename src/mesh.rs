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

fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 {x, y}
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 {x, y, z}
}

fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 {x, y, z, w}
}

#[repr(C)]
pub struct Vertex {
    pos: Vec3,
    col: Vec4,
    uv: Vec2,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
}

impl Mesh {
    pub fn new() -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let x = 0.5;
        let y = 0.5;
        vertices.push(Vertex {
            pos: vec3(x, y, 0.0),
            col: vec4(1.0, 0.0, 0.0, 1.0),
            uv: vec2(1.0, 0.0),
        }); // top right
        let x = 0.5;
        let y = -0.5;
        vertices.push(Vertex {
            pos: vec3(x, y, 0.0),
            col: vec4(1.0, 1.0, 0.0, 1.0),
            uv: vec2(1.0, 1.0),
        }); // bottom right
        let x = -0.5;
        let y = -0.5;
        vertices.push(Vertex {
            pos: vec3(x, y, 0.0),
            col: vec4(0.0, 0.0, 1.0, 1.0),
            uv: vec2(0.0, 1.0),
        }); // bottom left
        let x = -0.5;
        let y = 0.5;
        vertices.push(Vertex {
            pos: vec3(x, y, 0.0),
            col: vec4(0.0, 1.0, 0.0, 1.0),
            uv: vec2(0.0, 0.0),
        }); // top left

        indices.push(0);
        indices.push(1);
        indices.push(3);
        indices.push(1);
        indices.push(2);
        indices.push(3);
            
        Mesh {
            vertices,
            indices,
        }
    }

}