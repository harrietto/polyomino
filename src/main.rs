use macroquad::prelude::*;
use polyomino;

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
    let test = polyomino::Piece::new(vec![[0, 0], [0, 1], [1, 0]], [0, 0], RED);
    test.setup_texture();

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(screen_width() / 2.0 - 300.0, screen_height() / 2.0 - 200.0, 600.0, 400.0, 3.0, GRAY);

        test.draw(Vec2::new(200.0, 200.0));

        next_frame().await
    }
}