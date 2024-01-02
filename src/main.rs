//#![windows_subsystem = "windows"]

use miniquad::{self, conf::Platform, conf::Conf};

pub const WIDTH0: i32 = 1280;
pub const HEIGHT0: i32 = 800;

mod stage;
mod assets;
mod mesh;
mod shaders;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Computer Graphics From Scratch".to_owned(),
        window_width: WIDTH0,
        window_height: HEIGHT0,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(1);
    conf
}

fn main() {
    miniquad::start(window_conf(), move || Box::new(stage::Stage::new()));
}
