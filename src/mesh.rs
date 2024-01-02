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
pub struct Vertex {
    pos: Vec3,
    col: Vec4,
    uv: Vec2,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new_main() -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let mut idx = 0;

        let x = 1.0;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            col: Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0},
            uv: Vec2 {x: 1.0, y: 0.0},
        }); // top right
        let x = 1.0;
        let y = 1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            col: Vec4 { x: 1.0, y: 1.0, z: 0.0, w: 1.0},
            uv: Vec2 {x: 1.0, y: 1.0},
        }); // bottom right
        let x = -1.0;
        let y = 1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            col: Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0},
            uv: Vec2 {x: 0.0, y: 1.0},
        }); // bottom left
        let x = -1.0;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            col: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 1.0},
            uv: Vec2 {x: 0.0, y: 0.0},
        }); // top left

        indices.push(4 * idx);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 3);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 2);
        indices.push(4 * idx + 3);

        idx = idx + 1;
            
        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }

}