mod game;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Polyomino".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = game::AppState::new();

    loop {
        app_state.update();
        next_frame().await;
    }
}