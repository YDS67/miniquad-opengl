use miniquad::*;
use glam::{vec3, Mat4};
use image::{self, EncodableLayout};

use std::thread::sleep;
use std::time::Duration;

const FT_DESIRED: f32 = 0.01666666666667;

use crate::assets;
use crate::mesh;
use crate::shaders;

#[derive(Debug, Clone)]
struct Proj {
    mvp: Mat4,
}

impl Proj {
    fn new(angle_x: f32, angle_y: f32) -> Proj {
        let proj = Mat4::perspective_rh_gl(
            45.0/180.0*3.14159,
            crate::WIDTH0 as f32/crate::HEIGHT0 as f32,
            0.1,
            10.0,
        );

        let rotx = Mat4::from_rotation_y(angle_x);
        let roty = Mat4::from_rotation_z(angle_y);

        let view = Mat4::look_to_lh(
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
        );
        let mvp = proj * view * rotx * roty;
        Proj { mvp }
    }
}

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,

    mesh: mesh::Mesh,
    pipeline: Pipeline,
    bindings: Bindings,

    time_state: TimeState,
    input_state: InputState,
    screen_state: ScreenState,

    uniforms: shaders::UniformsMain,
    angles: (f32, f32),
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let assets = assets::Assets::load();

        let mesh = mesh::Mesh::new();

        let pixels = assets.tex;
        let dims = pixels.dimensions();

        let params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::None,
            width: dims.0,
            height: dims.1,
            allocate_mipmaps: true,
        };
        let texture = ctx.new_texture_from_data_and_format(pixels.as_bytes(), params);
        ctx.texture_generate_mipmaps(texture);

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh.vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh.indices),
        );

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
                VertexAttribute::new("col", VertexFormat::Float4),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        let proj = Proj::new(0.0, 0.0);

        Stage {
            ctx,

            mesh,
            pipeline,
            bindings,

            time_state: TimeState::init(),
            input_state: InputState::init(),
            screen_state: ScreenState::init(),

            uniforms: shaders::UniformsMain{
                mvp: proj.mvp,
            },

            angles: (0.0, 0.0),
        }
    }

    fn frame_time(&mut self) {
        self.time_state.frame_time = self.time_state.last_frame.elapsed().as_secs_f32();
        if self.time_state.frame_time < FT_DESIRED {
            sleep(Duration::from_secs_f32(
                FT_DESIRED - self.time_state.frame_time,
            ));
        }
        self.time_state.frame_time = self.time_state.last_frame.elapsed().as_secs_f32();
        self.time_state.fps = (1. / self.time_state.frame_time).floor() as i32 + 1;
    }

    fn show_data(&mut self) {
        if self.time_state.tick_count.overflowing_rem(60).0 == 0 {
            println!("FPS: {}, Ticks: {}, Frames: {}, Width: {:.0}, Height: {:.0}", 
                self.time_state.fps, self.time_state.tick_count, self.time_state.frame_count,
                self.screen_state.width, self.screen_state.height);
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.frame_time();
        self.show_data();

        if self.input_state.keys.esc {
            miniquad::window::quit()
        }

        if self.input_state.keys.a {
            self.angles.0 += 4.0*self.time_state.frame_time;
        }
        if self.input_state.keys.d {
            self.angles.0 -= 4.0*self.time_state.frame_time;
        }

        if self.input_state.keys.w {
            self.angles.1 += 4.0*self.time_state.frame_time;
        }
        if self.input_state.keys.s {
            self.angles.1 -= 4.0*self.time_state.frame_time;
        }

        self.time_state.tick_count += 1;
    }

    fn draw(&mut self) {
        self.ctx
            .begin_default_pass(miniquad::PassAction::clear_color(0.8, 0.8, 0.8, 1.0000000));

        self.ctx.apply_pipeline(&self.pipeline);

        self.ctx.apply_bindings(&self.bindings);

        self.uniforms.mvp = Proj::new(self.angles.0, self.angles.1).mvp;

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&self.uniforms));
        self.ctx.draw(0, self.mesh.num * 6, 1);

        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        self.time_state.last_frame = Some(std::time::Instant::now()).unwrap();

        self.time_state.frame_count += 1;
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input_state.keys.read_key(keycode, true)
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.keys.read_key(keycode, false)
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.input_state.mouse.x = x;
        self.input_state.mouse.y = y;
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.input_state.mouse.left = true;
        }
        if button == MouseButton::Right {
            self.input_state.mouse.right = true;
        }
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.input_state.mouse.left = false;
        }
        if button == MouseButton::Right {
            self.input_state.mouse.right = false;
        }
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.screen_state.width = width;
        self.screen_state.height = height;
    }
}

struct ScreenState {
    width: f32,
    height: f32,
}

impl ScreenState {
    fn init() -> ScreenState {
        ScreenState {
            width: crate::WIDTH0 as f32,
            height: crate::HEIGHT0 as f32,
        }
    }
}

struct TimeState {
    last_frame: std::time::Instant,
    frame_time: f32,
    tick_count: u128,
    frame_count: u128,
    fps: i32,
}

impl TimeState {
    fn init() -> TimeState {
        TimeState {
            last_frame: Some(std::time::Instant::now()).unwrap(),
            frame_time: 1.0 / 60.0,
            tick_count: 0,
            frame_count: 0,
            fps: 60,
        }
    }
}

pub struct KeysState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub k: bool,
    pub l: bool,
    pub f: bool,
    pub esc: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub enter: bool,
}

impl KeysState {
    fn read_key(&mut self, keycode: KeyCode, state: bool) {
        if keycode == KeyCode::W {
            self.w = state
        }
        if keycode == KeyCode::S {
            self.s = state
        }
        if keycode == KeyCode::Left {
            self.left = state
        }
        if keycode == KeyCode::Right {
            self.right = state
        }
        if keycode == KeyCode::A {
            self.a = state
        }
        if keycode == KeyCode::D {
            self.d = state
        }
        if keycode == KeyCode::Down {
            self.down = state
        }
        if keycode == KeyCode::Up {
            self.up = state
        }
        if keycode == KeyCode::Space {
            self.space = state
        }
        if keycode == KeyCode::Escape {
            self.esc = state
        }
        if keycode == KeyCode::Enter {
            self.enter = state
        }
        if keycode == KeyCode::K {
            self.k = state
        }
        if keycode == KeyCode::L {
            self.l = state
        }
        if keycode == KeyCode::Q {
            self.q = state
        }
        if keycode == KeyCode::E {
            self.e = state
        }
        if keycode == KeyCode::F {
            self.f = state
        }
    }
}

pub struct MouseState {
    pub left: bool,
    pub right: bool,
    pub moving: bool,
    pub x: f32,
    pub y: f32,
}

pub struct InputState {
    pub keys: KeysState,
    pub mouse: MouseState,
}

impl InputState {
    fn init() -> InputState {
        InputState {
            keys: KeysState {
                w: false,
                a: false,
                s: false,
                d: false,
                q: false,
                e: false,
                k: false,
                l: false,
                f: false,
                left: false,
                right: false,
                up: false,
                down: false,
                space: false,
                enter: false,
                esc: false,
            },
            mouse: MouseState {
                left: false,
                right: false,
                moving: false,
                x: 0.0,
                y: 0.0,
            },
        }
    }
}