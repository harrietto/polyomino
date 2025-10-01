use macroquad::prelude::*;

pub struct AppState {
    current_scene: Scene,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            current_scene: Scene::MainMenu(MainMenu::new()),
        }
    }

    pub fn update(&mut self) {
        if self.current_scene.update() {
            self.current_scene = self.current_scene.next_scene();
        }
    }
}

enum Scene {
    MainMenu(MainMenu),
    Settings(Settings),
    Game(Game)
}

impl Scene {
    fn next_scene(&self) -> Self {
        match self {
            Scene::MainMenu(_) => Scene::Settings(Settings::new()),
            Scene::Settings(_) => Scene::Game(Game::new()),
            Scene::Game(_) => Scene::MainMenu(MainMenu::new()),
        }
    }
    
    fn update(&mut self) -> bool {
        match self {
            Scene::MainMenu(main_menu) => {main_menu.update(); main_menu.is_next_scene()},
            Scene::Settings(settings) => {settings.update(); settings.is_next_scene()},
            Scene::Game(game) => {game.update(); game.is_next_scene()},
        }
    }
}

trait SceneBehavior {
    fn update(&mut self) {}
    fn is_next_scene(&self) -> bool {false}
}

struct MainMenu {
}

impl MainMenu {
    fn new() -> Self {
        MainMenu {}
    }
}

impl SceneBehavior for MainMenu {
    fn update(&mut self) {
        clear_background(BLACK);
        let text = "Main Menu. Press space to continue";
        let text_dimensions = measure_text(text, None, 30, 1.0);
        draw_text(text, screen_width() / 2.0 - text_dimensions.width / 2.0, screen_height() / 2.0 - text_dimensions.height / 2.0, 30.0, WHITE);
    }

    fn is_next_scene(&self) -> bool {
        is_key_pressed(KeyCode::Space)
    }
}

struct Settings {
}

impl Settings {
    fn new() -> Self {
        Settings {}
    }
}

impl SceneBehavior for Settings {
    fn update(&mut self) {
        clear_background(BLACK);
        let text = "Settings. Press space to continue";
        let text_dimensions = measure_text(text, None, 30, 1.0);
        draw_text(text, screen_width() / 2.0 - text_dimensions.width / 2.0, screen_height() / 2.0 - text_dimensions.height / 2.0, 30.0, WHITE);
    }

    fn is_next_scene(&self) -> bool {
        is_key_pressed(KeyCode::Space)
    }
}

struct Game {
    circle_radius: f32,
    top_left_pos: Vec2,
    cursor: polyomino::Cursor,
    pieces: [polyomino::Piece; 12],
    piece_icons: Vec<polyomino::PieceIcon>,
    spaces: [[Option<usize>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize],
    movable_piece: Option<usize>,
    is_next_scene: bool,
}

impl Game {
    fn new() -> Self {
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
        let piece_icons = pieces.iter().enumerate().map(|(i, piece)|
                if i < 6 {
                    polyomino::PieceIcon::new(piece, 10.0, Vec2::new((screen_width() - 80.0 * 6.0) / 2.0 + (i as f32 * 80.0), screen_height() - 210.0))
                } else { 
                    polyomino::PieceIcon::new(piece, 10.0, Vec2::new((screen_width() - 80.0 * 6.0) / 2.0 + ((i - 6) as f32 * 80.0), screen_height() - 210.0 + 80.0))
                }
            ).collect::<Vec<polyomino::PieceIcon>>();
        for icon in &piece_icons {
            icon.setup_texture();
        }
        let spaces: [[Option<usize>; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize] = [[None; polyomino::BOARD_DIMENSIONS[0] as usize]; polyomino::BOARD_DIMENSIONS[1] as usize];
        let movable_piece: Option<usize> = None;
        if let Some(index) = movable_piece {
            cursor.set_pos(pieces[index].get_pos());
        }
        
        Game {
            circle_radius,
            top_left_pos,
            cursor,
            pieces,
            piece_icons,
            spaces,
            movable_piece,
            is_next_scene: false,
        }
    }
    
    fn check_complete(&self) -> bool {
        for row in self.spaces {
            for space in row {
                if space == None {
                    return false;
                }
            }
        }
        true
    }
}

impl SceneBehavior for Game {
    fn is_next_scene(&self) -> bool {
        self.is_next_scene
    }

    fn update(&mut self) {
        clear_background(BLACK);

        polyomino::draw_circle_grid(self.top_left_pos.x, self.top_left_pos.y, polyomino::BOARD_DIMENSIONS[1], polyomino::BOARD_DIMENSIONS[0], self.circle_radius, Color::from_hex(0x2b2b2b));

        for (i, piece) in self.pieces.iter().enumerate() {
            if Some(i) == self.movable_piece {
                continue;
            }
            piece.draw();
        }
        if let Some(index) = self.movable_piece {
            self.pieces[index].draw();
        }

        draw_rectangle(0.0, 0.0, self.top_left_pos.x - self.circle_radius * 2.0, screen_height(), Color::from_hex(0x1c1c1c));
        draw_rectangle(screen_width() - (self.top_left_pos.x - self.circle_radius * 2.0), 0.0, self.top_left_pos.x - self.circle_radius * 2.0, screen_height(), Color::from_hex(0x1c1c1c));
        draw_rectangle(0.0, 0.0, screen_width(), self.top_left_pos.y - self.circle_radius * 2.0, Color::from_hex(0x1c1c1c));
        draw_rectangle(0.0, self.top_left_pos.y + self.circle_radius * 2.0 * 6.0, screen_width(), screen_height() - (self.top_left_pos.y + self.circle_radius * 2.0 * 6.0), Color::from_hex(0x1c1c1c));

        for (i, icon) in self.piece_icons.iter().enumerate() {
            icon.draw(&self.pieces[i]);
        }
        self.cursor.draw();

        if is_key_pressed(KeyCode::Right) {
            self.cursor.translate([1, 0]);
            if let Some(index) = self.movable_piece {
                self.pieces[index].translate(1, 0);
            }
        }
        if is_key_pressed(KeyCode::Left) {
            self.cursor.translate([-1, 0]);
            if let Some(index) = self.movable_piece {
                self.pieces[index].translate(-1, 0);
                
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.cursor.translate([0, -1]);
            if let Some(index) = self.movable_piece {
                self.pieces[index].translate(0, -1);
            }
        }
        if is_key_pressed(KeyCode::Down) {
            self.cursor.translate([0, 1]);
            if let Some(index) = self.movable_piece {
                self.pieces[index].translate(0, 1);
            }
        }
        if is_key_pressed(KeyCode::W) {
            if let Some(index) = self.movable_piece {
                self.pieces[index].rotate(true);
            }
        }
        if is_key_pressed(KeyCode::Q) {
            if let Some(index) = self.movable_piece {
                self.pieces[index].rotate(false);
            }
        }
        if is_key_pressed(KeyCode::E) {
            if let Some(index) = self.movable_piece {
                self.pieces[index].flip();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            if let Some(index) = self.movable_piece {
                match self.pieces[index].lock(&mut self.spaces) {
                    Ok(()) => {self.movable_piece = None; self.piece_icons[index].deselect();},
                    Err(()) => (),
                };
            } else if self.movable_piece == None {
                if let Some(index) = self.spaces[self.cursor.get_pos()[1] as usize][self.cursor.get_pos()[0] as usize] {
                    self.movable_piece = Some(index);
                    self.pieces[index].unlock(&mut self.spaces);
                    self.piece_icons[index].select();
                    self.cursor.set_pos(self.pieces[index].get_pos());
                }
            }

            if self.check_complete() {
                println!("Puzzle complete!");
            }
        }
        if  is_key_pressed(KeyCode::Escape) {
            if let Some(index) = self.movable_piece {
                self.pieces[index].deselect();
                self.pieces[index].reset_rotation_and_flipping();
                self.piece_icons[index].deselect();
            }
            self.movable_piece = None;
        }
        if is_key_pressed(KeyCode::Key1) {
            if !self.pieces[0].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(0);
                self.piece_icons[0].select();
                self.pieces[0].select();
                self.pieces[0].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key2) {
            if !self.pieces[1].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(1);
                self.piece_icons[1].select();
                self.pieces[1].select();
                self.pieces[1].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key3) {
            if !self.pieces[2].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(2);
                self.piece_icons[2].select();
                self.pieces[2].select();
                self.pieces[2].set_pos(self.cursor.get_pos());
            }
        }
        if is_key_pressed(KeyCode::Key4) {
            if !self.pieces[3].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(3);
                self.piece_icons[3].select();
                self.pieces[3].select();
                self.pieces[3].set_pos(self.cursor.get_pos());
            }
        }
        if is_key_pressed(KeyCode::Key5) {
            if !self.pieces[4].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(4);
                self.piece_icons[4].select();
                self.pieces[4].select();
                self.pieces[4].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key6) {
            if !self.pieces[5].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(5);
                self.piece_icons[5].select();
                self.pieces[5].select();
                self.pieces[5].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key7) {
            if !self.pieces[6].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(6);
                self.piece_icons[6].select();
                self.pieces[6].select();
                self.pieces[6].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key8) {
            if !self.pieces[7].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(7);
                self.piece_icons[7].select();
                self.pieces[7].select();
                self.pieces[7].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key9) {
            if !self.pieces[8].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(8);
                self.piece_icons[8].select();
                self.pieces[8].select();
                self.pieces[8].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Key0) {
            if !self.pieces[9].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(9);
                self.piece_icons[9].select();
                self.pieces[9].select();
                self.pieces[9].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Minus) {
            if !self.pieces[10].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(10);
                self.piece_icons[10].select();
                self.pieces[10].select();
                self.pieces[10].set_pos(self.cursor.get_pos()); 
            }
        }
        if is_key_pressed(KeyCode::Equal) {
            if !self.pieces[11].locked {
                if let Some(index) = self.movable_piece {
                    self.pieces[index].deselect();
                    self.piece_icons[index].deselect();
                }
                self.movable_piece = Some(11);
                self.piece_icons[11].select();
                self.pieces[11].select();
                self.pieces[11].set_pos(self.cursor.get_pos()); 
            }
        }
    }
}