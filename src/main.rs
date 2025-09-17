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
    let circle_radius = 20.0;
    let board_size = Vec2::new((polyomino::BOARD_DIMENSIONS[0]) as f32 * circle_radius * 2.0, (polyomino::BOARD_DIMENSIONS[1]) as f32 * circle_radius * 2.0);
    let outline_size = Vec2::new((polyomino::BOARD_DIMENSIONS[0] + 2) as f32 * circle_radius * 2.0, (polyomino::BOARD_DIMENSIONS[1] + 2) as f32 * circle_radius * 2.0);
    let mut cursor = polyomino::Cursor::new([0, 0], circle_radius, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
    let mut test = polyomino::Piece::new(vec![[0, 0], [0, 1], [1, 0]], [0, 0], circle_radius, [0, 0], Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), RED);
    test.setup_texture();
    let mut spaces: [[Option<&polyomino::Piece>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize] = [[None; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize];

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(screen_width() / 2.0 - outline_size.x / 2.0 - 5.0, screen_height() / 2.0 - outline_size.y / 2.0 - 5.0, outline_size.x + 10.0, outline_size.y + 10.0, 10.0, Color::from_hex(0x4d4d4d));

        polyomino::draw_circle_grid(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0, polyomino::BOARD_DIMENSIONS[1], polyomino::BOARD_DIMENSIONS[0], circle_radius, Color::from_hex(0x2b2b2b));

        test.draw();
        cursor.draw();

        if is_key_pressed(KeyCode::Right) {
            cursor.translate([1, 0]);
            test.translate(1, 0, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Left) {
            cursor.translate([-1, 0]);
            test.translate(-1, 0, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Up) {
            cursor.translate([0, -1]);
            test.translate(0, -1, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::Down) {
            cursor.translate([0, 1]);
            test.translate(0, 1, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), Vec2::new(screen_width() / 2.0 + board_size.x / 2.0, screen_height() / 2.0 + board_size.y / 2.0));
        }
        if is_key_pressed(KeyCode::X) {
            test.rotate(true);
        }
        if is_key_pressed(KeyCode::Z) {
            test.rotate(false);
        }
        if is_key_pressed(KeyCode::Enter) {
            test.lock(&mut spaces);
        }

        next_frame().await
    }
}