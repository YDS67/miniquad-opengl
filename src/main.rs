use macroquad::prelude::*;

mod assets;
mod settings;
mod shaders;
mod stage;
mod player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad + OpenGL".to_owned(),
        high_dpi: true,
        window_width: settings::WIDTH,
        window_height: settings::HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let ass = assets::Ass::load().await;

    let mut player = player::Player::new();

    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        stage::Stage::new(ctx, &ass)
    }
    .await;

    let t_par = TextParams {
        font_size: 30,
        font: Some(&ass.font_main),
        color: BLACK,
        ..Default::default()
    };

    loop {
        clear_background(Color::from_rgba(135, 206, 235, 255));

        player.walk();

        //Render some primitives in camera space

        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            gl.quad_context
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::Uniforms {
                    playerpos: (player.position.x, player.position.y, player.position.z),
                    playerdir: (player.position.a, player.position.b),
                }));
            gl.quad_context.draw(0, 36, 1);

            gl.quad_context.end_render_pass();
        }
        
        draw_words(&t_par, &player);

        next_frame().await
    }
}

fn draw_words(t_par: &TextParams, player: &player::Player) {
    draw_rectangle(10.0, 10.0, 220.0, 140.0, WHITE);
    draw_rectangle_lines(10.0, 10.0, 220.0, 140.0, 4.0, BLACK);
    draw_text_ex("Open GL Render", 20.0, 40.0, t_par.clone());
    let fps = get_fps();
    let mut fps_display = fps;
    if fps > 50 && fps < 70 {
        fps_display = 60
    }
    draw_text_ex(
        &format!("FPS is {}", fps_display),
        20.0,
        70.0,
        t_par.to_owned(),
    );
    draw_text_ex("Player position:", 20.0, 100.0, t_par.to_owned());
    draw_text_ex(
        &format!("({:.1},{:.1})", player.position.x, player.position.y),
        20.0,
        130.0,
        t_par.to_owned(),
    );
}