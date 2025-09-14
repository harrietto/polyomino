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
    let mut test = polyomino::Piece::new(vec![[0, 0], [0, 1], [1, 0]], [0, 0], 20.0, [0, 0], Vec2::new(screen_width() / 2.0 - 220.0, screen_height() / 2.0 - 100.0), RED);
    test.setup_texture();

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(screen_width() / 2.0 - 225.0, screen_height() / 2.0 - 105.0, 450.0, 210.0, 10.0, GRAY);

        polyomino::draw_circle_grid(screen_width() / 2.0 - 220.0, screen_height() / 2.0 - 100.0, 5, 11, 20.0, Color::from_hex(0x2b2b2b));

        test.draw();

        if is_key_pressed(KeyCode::Right) {
            test.translate(1, 0);
        }
        if is_key_pressed(KeyCode::Left) {
            test.translate(-1, 0);
        }
        if is_key_pressed(KeyCode::Up) {
            test.translate(0, -1);
        }
        if is_key_pressed(KeyCode::Down) {
            test.translate(0, 1);
        }
        if is_key_pressed(KeyCode::Z) {
            test.rotate(true);
        }
        if is_key_pressed(KeyCode::X) {
            test.rotate(false);
        }

        next_frame().await
    }
}