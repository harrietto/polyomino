use macroquad::prelude::*;

pub struct Piece {
    circle_positions: Vec<[u32; 2]>,
    centre_of_rotation: [u32; 2],
    colour: macroquad::color::Color,
    render_target: macroquad::texture::RenderTarget,
    render_camera: macroquad::camera::Camera2D,
}

impl Piece {
    pub fn new(circle_positions: Vec<[u32; 2]>, centre_of_rotation: [u32; 2], colour: macroquad::color::Color) -> Piece {
        let render_target = render_target(256, 256);
        render_target.texture.set_filter(FilterMode::Nearest);
        let render_camera = Camera2D {
            render_target: Some(render_target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0))
        };

        Piece {
            circle_positions,
            centre_of_rotation,
            colour,
            render_target,
            render_camera,
        }
    }

    pub fn setup_texture(&self) {
        set_camera(&self.render_camera);

        for pos in &self.circle_positions {
            draw_circle(((pos[0] + 1) * 40) as f32, ((pos[1] + 1) * 40) as f32,  20.0, self.colour);
            println!("TEST {}, {}", pos[0], pos[1]);
        }

        set_default_camera();
    }

    pub fn draw(&self, position: Vec2) {
        draw_texture_ex(
            &self.render_target.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(256.0, 256.0)),
                ..Default::default()
            }
        );
    }
}