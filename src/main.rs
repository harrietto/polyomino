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
    let board_dimensions = [11, 5];
    let circle_radius = 20.0;
    let board_size = Vec2::new((board_dimensions[0]) as f32 * circle_radius * 2.0, (board_dimensions[1]) as f32 * circle_radius * 2.0);
    let outline_size = Vec2::new((board_dimensions[0] + 2) as f32 * circle_radius * 2.0, (board_dimensions[1] + 2) as f32 * circle_radius * 2.0);
    let mut test = polyomino::Piece::new(vec![[0, 0], [0, 1], [1, 0]], [0, 0], circle_radius, [0, 0], Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), RED);
    test.setup_texture();

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(screen_width() / 2.0 - outline_size.x / 2.0 - 5.0, screen_height() / 2.0 - outline_size.y / 2.0 - 5.0, outline_size.x + 10.0, outline_size.y + 10.0, 10.0, GRAY);

        polyomino::draw_circle_grid(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0, board_dimensions[1], board_dimensions[0], circle_radius, Color::from_hex(0x2b2b2b));

        test.draw();

        if is_key_pressed(KeyCode::Right) {
            test.translate(1, 0, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Left) {
            test.translate(-1, 0, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Up) {
            test.translate(0, -1, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Down) {
            test.translate(0, 1, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::X) {
            test.rotate(true);
        }
        if is_key_pressed(KeyCode::Z) {
            test.rotate(false);
        }

        next_frame().await
    }
}