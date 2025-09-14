use macroquad::prelude::*;

pub struct Piece {
    circle_positions: Vec<[u32; 2]>,
    centre_of_rotation: [u32; 2],
    circle_radius: f32,
    position: Vec2,
    rotation: f32,
    colour: macroquad::color::Color,
    render_target: macroquad::texture::RenderTarget,
    render_camera: macroquad::camera::Camera2D,
}

impl Piece {
    pub fn new(circle_positions: Vec<[u32; 2]>, centre_of_rotation: [u32; 2], circle_radius: f32, grid_pos: [u32; 2], top_left_pos: Vec2, colour: macroquad::color::Color) -> Piece {
        let position = Vec2::new(top_left_pos.x + grid_pos[0] as f32 * circle_radius * 2.0, top_left_pos.y + grid_pos[1] as f32 * circle_radius * 2.0);
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
            position,
            rotation: 0.0,
            colour,
            render_target,
            render_camera,
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
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(256.0, 256.0)),
                rotation: self.rotation.to_radians(),
                pivot: Some(Vec2::new(self.position.x + (self.centre_of_rotation[0] * 2 + 1) as f32 * self.circle_radius, self.position.y +  (self.centre_of_rotation[1] * 2 + 1) as f32 * self.circle_radius)),
                ..Default::default()
            }
        );
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.position.x += x as f32 * self.circle_radius * 2.0;
        self.position.y += y as f32 * self.circle_radius * 2.0;
    }

    pub fn rotate(&mut self, clockwise: bool) {
        self.rotation += if clockwise { 90.0 } else { -90.0 };
    }
}

pub fn draw_circle_grid(x: f32, y: f32, rows: u32, cols: u32, circle_radius: f32, colour: macroquad::color::Color) {
    for i in 0..cols {
        for j in 0..rows {
            draw_circle(x + (i as f32 + 0.5) * circle_radius * 2.0, y + (j as f32 + 0.5) * circle_radius * 2.0, circle_radius, colour);
        }
    }
}