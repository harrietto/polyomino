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

    let mut cursor = polyomino::Cursor::new([0, 0], circle_radius, Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0));

    let red = polyomino::Piece::new(0, vec![[0, 1], [0, 2], [1, 1], [1, 0]], [0, 1], circle_radius, [0, 1], Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), RED);
    red.setup_texture();
    let blue = polyomino::Piece::new(1, vec![[0, 0], [0, 1], [0, 2], [1, 0]], [0, 0], circle_radius, [6, 2], Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0), BLUE);
    blue.setup_texture();

    let mut pieces = [red, blue];
    let mut spaces: [[Option<usize>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize] = [[None; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize];
    let mut movable_piece: Option<usize> = Some(0);
    if let Some(index) = movable_piece {
        cursor.set_pos(pieces[index].get_pos());
    }

    pieces[1].lock(&mut spaces).unwrap();

    loop {
        clear_background(BLACK);

        draw_rectangle_lines(screen_width() / 2.0 - outline_size.x / 2.0 - 5.0, screen_height() / 2.0 - outline_size.y / 2.0 - 5.0, outline_size.x + 10.0, outline_size.y + 10.0, 10.0, Color::from_hex(0x4d4d4d));

        polyomino::draw_circle_grid(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0, polyomino::BOARD_DIMENSIONS[1], polyomino::BOARD_DIMENSIONS[0], circle_radius, Color::from_hex(0x2b2b2b));

        for piece in &pieces {
            piece.draw();
        }
        cursor.draw();

        if is_key_pressed(KeyCode::Right) {
            cursor.translate([1, 0]);
            if let Some(index) = movable_piece {
                pieces[index].translate(1, 0);
            }
        }
        if is_key_pressed(KeyCode::Left) {
            cursor.translate([-1, 0]);
            if let Some(index) = movable_piece {
                pieces[index].translate(-1, 0);
                
            }
        }
        if is_key_pressed(KeyCode::Up) {
            cursor.translate([0, -1]);
            if let Some(index) = movable_piece {
                pieces[index].translate(0, -1);
            }
        }
        if is_key_pressed(KeyCode::Down) {
            cursor.translate([0, 1]);
            if let Some(index) = movable_piece {
                pieces[index].translate(0, 1);
            }
        }
        if is_key_pressed(KeyCode::X) {
            if let Some(index) = movable_piece {
                pieces[index].rotate(true);
            }
        }
        if is_key_pressed(KeyCode::Z) {
            if let Some(index) = movable_piece {
                pieces[index].rotate(false);
            }
        }
        if is_key_pressed(KeyCode::Enter) {
            if let Some(index) = movable_piece {
                match pieces[index].lock(&mut spaces) {
                    Ok(()) => movable_piece = None,
                    Err(()) => (),
                };
            } else if movable_piece == None {
                if let Some(index) = spaces[cursor.get_pos()[1] as usize][cursor.get_pos()[0] as usize] {
                    movable_piece = Some(index);
                    pieces[index].unlock(&mut spaces);
                    cursor.set_pos(pieces[index].get_pos());
                }
            }
        }


        next_frame().await
    }
}