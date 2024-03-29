#![windows_subsystem = "windows"]

use miniquad::{self, conf::Platform, conf::Conf};

pub const WIDTH0: i32 = 900;
pub const HEIGHT0: i32 = 720;

mod stage;
mod assets;
mod mesh;
mod shaders;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Miniquad example, WASD to rotate, Esc to quit".to_owned(),
        window_width: WIDTH0,
        window_height: HEIGHT0,
        platform: Platform::default(),
        window_resizable: true,
        ..Default::default()
    };
    conf.platform.swap_interval = Some(-1);
    conf
}

fn main() {
    miniquad::start(window_conf(), move || Box::new(stage::Stage::new()));
}
