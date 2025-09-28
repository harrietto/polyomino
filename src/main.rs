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

fn check_complete(spaces: &[[Option<usize>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize]) -> bool {
    for row in spaces {
        for space in row {
            if space == &None {
                return false;
            }
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let circle_radius = 20.0;
    let board_size = Vec2::new((polyomino::BOARD_DIMENSIONS[0]) as f32 * circle_radius * 2.0, (polyomino::BOARD_DIMENSIONS[1]) as f32 * circle_radius * 2.0);
    let top_left_pos = Vec2::new(screen_width() / 2.0 - board_size.x / 2.0, screen_height() / 2.0 - board_size.y / 2.0 - 100.0);

    let mut cursor = polyomino::Cursor::new([0, 0], circle_radius, top_left_pos);
    
    let mut pieces = [
        polyomino::Piece::new(0, vec![[0, 1], [1, 1], [2, 1], [0, 0]], [0, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0x7c8ee6)),
        polyomino::Piece::new(1, vec![[1, 1], [2, 1], [0, 0], [1, 0]], [1, 0], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xf26a66)),
        polyomino::Piece::new(2, vec![[0, 1], [1, 1], [2, 1], [0, 0], [1, 0]], [1, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0x72d6ae)),
        polyomino::Piece::new(3, vec![[0, 2], [1, 2], [2, 2], [0, 1], [0, 0]], [0, 2], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0x36a7e3)),
        polyomino::Piece::new(4, vec![[0, 1], [1, 1], [2, 1], [0, 0], [2, 0]], [1, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xa8d162)),
        polyomino::Piece::new(5, vec![[1, 2], [0, 1], [1, 1], [2, 1], [2, 1], [2, 0]], [1, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xed9c51)),
        polyomino::Piece::new(6, vec![[0, 3], [0, 2], [0, 1], [1, 1], [1, 0]], [0, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xe3a3c5)),
        polyomino::Piece::new(7, vec![[0, 2], [1, 2], [2, 2], [1, 1]], [1, 2], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0x369c6f)),
        polyomino::Piece::new(8, vec![[1, 3], [1, 2], [0, 1], [1, 1], [1, 0]], [1, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xf2cf4e)),
        polyomino::Piece::new(9, vec![[1, 2], [2, 2], [0, 1], [1, 1], [0, 0]], [1, 1], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xa477d1)),
        polyomino::Piece::new(10, vec![[1, 3], [2, 3], [1, 2], [1, 1], [1, 0]], [1, 3], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0xb33d3d)),
        polyomino::Piece::new(11, vec![[1, 2], [2, 2], [1, 1]], [1, 2], circle_radius, top_left_pos, macroquad::color::Color::from_hex(0x75ccd1)),
    ];

    for piece in &mut pieces {
        piece.setup_texture();
    }
    let mut piece_icons = pieces.iter().enumerate().map(|(i, piece)|
            if i < 6 {
                polyomino::PieceIcon::new(piece, 10.0, Vec2::new((screen_width() - 80.0 * 6.0) / 2.0 + (i as f32 * 80.0), screen_height() - 210.0))
            } else { 
                polyomino::PieceIcon::new(piece, 10.0, Vec2::new((screen_width() - 80.0 * 6.0) / 2.0 + ((i - 6) as f32 * 80.0), screen_height() - 210.0 + 80.0))
            }
        ).collect::<Vec<polyomino::PieceIcon>>();
    for icon in &piece_icons {
        icon.setup_texture();
    }
    let mut spaces: [[Option<usize>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize] = [[None; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize];
    let mut movable_piece: Option<usize> = None;
    if let Some(index) = movable_piece {
        cursor.set_pos(pieces[index].get_pos());
    }

    loop {
        clear_background(BLACK);

        polyomino::draw_circle_grid(top_left_pos.x, top_left_pos.y, polyomino::BOARD_DIMENSIONS[1], polyomino::BOARD_DIMENSIONS[0], circle_radius, Color::from_hex(0x2b2b2b));

        for (i, piece) in pieces.iter().enumerate() {
            if Some(i) == movable_piece {
                continue;
            }
            piece.draw();
        }
        if let Some(index) = movable_piece {
            pieces[index].draw();
        }

        draw_rectangle(0.0, 0.0, top_left_pos.x - circle_radius * 2.0, screen_height(), Color::from_hex(0x1c1c1c));
        draw_rectangle(screen_width() - (top_left_pos.x - circle_radius * 2.0), 0.0, top_left_pos.x - circle_radius * 2.0, screen_height(), Color::from_hex(0x1c1c1c));
        draw_rectangle(0.0, 0.0, screen_width(), top_left_pos.y - circle_radius * 2.0, Color::from_hex(0x1c1c1c));
        draw_rectangle(0.0, top_left_pos.y + circle_radius * 2.0 * 6.0, screen_width(), screen_height() - (top_left_pos.y + circle_radius * 2.0 * 6.0), Color::from_hex(0x1c1c1c));

        for (i, icon) in piece_icons.iter().enumerate() {
            icon.draw(&pieces[i]);
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
        if is_key_pressed(KeyCode::W) {
            if let Some(index) = movable_piece {
                pieces[index].rotate(true);
            }
        }
        if is_key_pressed(KeyCode::Q) {
            if let Some(index) = movable_piece {
                pieces[index].rotate(false);
            }
        }
        if is_key_pressed(KeyCode::E) {
            if let Some(index) = movable_piece {
                pieces[index].flip();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            if let Some(index) = movable_piece {
                match pieces[index].lock(&mut spaces) {
                    Ok(()) => {movable_piece = None; piece_icons[index].deselect();},
                    Err(()) => (),
                };
            } else if movable_piece == None {
                if let Some(index) = spaces[cursor.get_pos()[1] as usize][cursor.get_pos()[0] as usize] {
                    movable_piece = Some(index);
                    pieces[index].unlock(&mut spaces);
                    piece_icons[index].select();
                    cursor.set_pos(pieces[index].get_pos());
                }
            }

            if check_complete(&spaces) {
                println!("Puzzle complete!");
            }
        }
        if  is_key_pressed(KeyCode::Escape) {
            if let Some(index) = movable_piece {
                pieces[index].deselect();
                pieces[index].reset_rotation_and_flipping();
                piece_icons[index].deselect();
            }
            movable_piece = None;
        }
        if is_key_pressed(KeyCode::Key1) {
            if !pieces[0].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(0);
                piece_icons[0].select();
                pieces[0].select();
                pieces[0].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key2) {
            if !pieces[1].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(1);
                piece_icons[1].select();
                pieces[1].select();
                pieces[1].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key3) {
            if !pieces[2].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(2);
                piece_icons[2].select();
                pieces[2].select();
                pieces[2].set_pos(cursor.get_pos());
            }
        }
        if is_key_pressed(KeyCode::Key4) {
            if !pieces[3].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(3);
                piece_icons[3].select();
                pieces[3].select();
                pieces[3].set_pos(cursor.get_pos());
            }
        }
        if is_key_pressed(KeyCode::Key5) {
            if !pieces[4].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(4);
                piece_icons[4].select();
                pieces[4].select();
                pieces[4].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key6) {
            if !pieces[5].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(5);
                piece_icons[5].select();
                pieces[5].select();
                pieces[5].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key7) {
            if !pieces[6].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(6);
                piece_icons[6].select();
                pieces[6].select();
                pieces[6].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key8) {
            if !pieces[7].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(7);
                piece_icons[7].select();
                pieces[7].select();
                pieces[7].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key9) {
            if !pieces[8].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(8);
                piece_icons[8].select();
                pieces[8].select();
                pieces[8].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key0) {
            if !pieces[9].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(9);
                piece_icons[9].select();
                pieces[9].select();
                pieces[9].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Minus) {
            if !pieces[10].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(10);
                piece_icons[10].select();
                pieces[10].select();
                pieces[10].set_pos(cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Equal) {
            if !pieces[11].locked {
                if let Some(index) = movable_piece {
                    pieces[index].deselect();
                    piece_icons[index].deselect();
                }
                movable_piece = Some(11);
                piece_icons[11].select();
                pieces[11].select();
                pieces[11].set_pos(cursor.get_pos()); 
            }
        }

        next_frame().await
    }
}