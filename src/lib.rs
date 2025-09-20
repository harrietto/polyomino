use macroquad::prelude::*;

pub const BOARD_DIMENSIONS: [u32; 2] = [11, 5];

pub struct Piece {
    circle_positions: Vec<[u32; 2]>,
    centre_of_rotation: [i32; 2],
    circle_radius: f32,
    top_left_pos: Vec2,
    position: [i32; 2],
    rotation: i32,
    locked: bool,
    colour: macroquad::color::Color,
    render_target: macroquad::texture::RenderTarget,
    render_camera: macroquad::camera::Camera2D,
    index: usize,
}

impl Piece {
    pub fn new(index: usize, circle_positions: Vec<[u32; 2]>, centre_of_rotation: [i32; 2], circle_radius: f32, grid_pos: [i32; 2], top_left_pos: Vec2, colour: macroquad::color::Color) -> Piece {
        let position = [grid_pos[0] - centre_of_rotation[0], grid_pos[1] - centre_of_rotation[1]];
        let render_target = render_target(256, 256);
        render_target.texture.set_filter(FilterMode::Linear);
        let render_camera = Camera2D {
            render_target: Some(render_target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
        };

        Piece {
            circle_positions,
            centre_of_rotation,
            circle_radius,
            top_left_pos,
            position,
            rotation: 0,
            colour,
            locked: false,
            render_target,
            render_camera,
            index,
        }
    }

    pub fn setup_texture(&self) {
        set_camera(&self.render_camera);

        for pos in &self.circle_positions {
            draw_circle((pos[0] as f32 + 0.5) * self.circle_radius * 2.0, self.render_target.texture.height() - (pos[1] as f32 + 0.5) * self.circle_radius * 2.0, self.circle_radius, self.colour);
        }

        set_default_camera();
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.render_target.texture,
            self.top_left_pos.x + self.position[0] as f32 * self.circle_radius * 2.0,
            self.top_left_pos.y + self.position[1] as f32 * self.circle_radius * 2.0,
            if self.locked { self.colour } else { Color::from_hex(0x808080) },
            DrawTextureParams {
                dest_size: Some(Vec2::new(256.0, 256.0)),
                rotation: (self.rotation as f32).to_radians(),
                pivot: Some(self.top_left_pos + Vec2::new(((self.position[0] + self.centre_of_rotation[0]) * 2 + 1) as f32 * self.circle_radius, ((self.position[1] + self.centre_of_rotation[1]) * 2 + 1) as f32 * self.circle_radius)),
                ..Default::default()
            }
        );
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        if self.locked == true {
            return;
        }

        if self.position[0] + self.centre_of_rotation[0] + x < 0 || self.position[0] + self.centre_of_rotation[0] + x > BOARD_DIMENSIONS[0] as i32 - 1 || self.position[1] + self.centre_of_rotation[1] + y < 0 || self.position[1] + self.centre_of_rotation[1] + y > BOARD_DIMENSIONS[1] as i32 - 1 {
            return;
        }

        self.position[0] = self.position[0] + x;
        self.position[1] = self.position[1] + y;
    }

    pub fn get_pos(&self) -> [i32; 2] {
        [self.position[0] + self.centre_of_rotation[0], self.position[1] + self.centre_of_rotation[1]]
    }

    pub fn rotate(&mut self, clockwise: bool) {
        if self.locked == true {
            return;
        }

        self.rotation += if clockwise { 90 } else { -90 };
    }

    fn get_rotated_pos(&self, pos: &[u32; 2]) -> [i32; 2] {
        match self.rotation % 360 {
            0 => [self.position[0] + pos[0] as i32, self.position[1] + pos[1] as i32],
            90 | -270 => [self.position[0] - pos[1] as i32, self.position[1] + pos[0] as i32],
            180 | -180 => [self.position[0] - pos[0] as i32, self.position[1] - pos[1] as i32],
            270 | -90 => [self.position[0] + pos[1] as i32, self.position[1] - pos[0] as i32],
            _ => panic!("Rotation not a multiple of 90 degrees"),
        }
    }

    pub fn lock(&mut self, spaces: &mut [[Option<usize>; BOARD_DIMENSIONS[0] as usize]; BOARD_DIMENSIONS[1] as usize]) -> Result<(), ()> {
        for pos in &self.circle_positions {
            let circle_pos = self.get_rotated_pos(&pos);

            if circle_pos[0] < 0 || circle_pos[1] < 0 || circle_pos[0] > BOARD_DIMENSIONS[0] as i32 - 1 || circle_pos[1] > BOARD_DIMENSIONS[1] as i32 - 1 || spaces[circle_pos[1] as usize][circle_pos[0] as usize] != None {
                return Err(());
            }
        }

        for pos in &self.circle_positions {
            let circle_pos = self.get_rotated_pos(&pos);

            spaces[circle_pos[1] as usize][circle_pos[0] as usize] = Some(self.index);
        }

        self.locked = true;

        Ok(())
    }

    pub fn unlock(&mut self, spaces: &mut [[Option<usize>; BOARD_DIMENSIONS[0] as usize]; BOARD_DIMENSIONS[1] as usize]) {
        for pos in &self.circle_positions {
            let circle_pos = self.get_rotated_pos(&pos);

            spaces[circle_pos[1] as usize][circle_pos[0] as usize] = None;
        }

        self.locked = false;
    }
}

pub struct Cursor {
    position: [i32; 2],
    circle_radius: f32,
    top_left_pos: Vec2,
}

impl Cursor {
    pub fn new(position: [i32; 2], circle_radius: f32, top_left_pos: Vec2) -> Cursor {
        Cursor {
            position,
            circle_radius,
            top_left_pos,
        }
    }

    pub fn set_pos(&mut self, position: [i32; 2]) {
        self.position = position;
    }

    pub fn get_pos(&self) -> [i32; 2] {
        self.position
    }
 
    pub fn translate(&mut self, change: [i32; 2]) {
        if self.position[0] + change[0] < 0 || self.position[0] + change[0] > BOARD_DIMENSIONS[0] as i32 - 1 || self.position[1] + change[1] < 0 || self.position[1] + change[1] > BOARD_DIMENSIONS[1] as i32 - 1 {
            return;
        }

        self.position = [self.position[0] + change[0], self.position[1] + change[1]];
    }

    pub fn draw(&self) {
        let draw_pos = Vec2::new(self.top_left_pos.x + (self.position[0] as f32 + 0.5) * self.circle_radius * 2.0, self.top_left_pos.y + (self.position[1] as f32 + 0.5) * self.circle_radius * 2.0);

        draw_circle_lines(draw_pos.x, draw_pos.y, self.circle_radius - 3.0, 3.0, WHITE);
    }
}

pub fn draw_circle_grid(x: f32, y: f32, rows: u32, cols: u32, circle_radius: f32, colour: macroquad::color::Color) {
    for i in 0..cols {
        for j in 0..rows {
            draw_circle(x + (i as f32 + 0.5) * circle_radius * 2.0, y + (j as f32 + 0.5) * circle_radius * 2.0, circle_radius, colour);
        }
    }
}