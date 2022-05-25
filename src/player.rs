use ggez::event::KeyCode;
use ggez::graphics::*;
use ggez::*;
use glam::Vec2;

use crate::config::{BLOCK_SIZE, PLAYER_SPEED};
use crate::inputs::Inputs;

pub struct Player {
    pub pos: Vec2,
    size: Vec2,
    speed: Vec2,
    screen_size: Vec2,
}

impl Player {
    pub fn new(screen_size: Vec2) -> Player {
        Player {
            pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(BLOCK_SIZE as f32, BLOCK_SIZE as f32),
            speed: Vec2::zero(),
            screen_size,
        }
    }

    pub fn update(&mut self, inputs: &Inputs, dt: f32) {
        let left = inputs.is_key_pressed(KeyCode::Q);
        let right = inputs.is_key_pressed(KeyCode::D);
        let up = inputs.is_key_pressed(KeyCode::Z);
        let down = inputs.is_key_pressed(KeyCode::S);
        if !left && !right {
            self.speed.set_x(0.0f32);
        } else if left && right {
            self.speed.set_x(0.0f32);
        } else if left {
            self.speed.set_x(-PLAYER_SPEED);
        } else if right {
            self.speed.set_x(PLAYER_SPEED);
        }
        if !up && !down {
            self.speed.set_y(0.0f32);
        } else if up && down {
            self.speed.set_y(0.0f32);
        } else if up {
            self.speed.set_y(-PLAYER_SPEED);
        } else if down {
            self.speed.set_y(PLAYER_SPEED);
        }

        self.pos += self.speed * dt;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let pos = self.world_to_screen_pos(self.pos);
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(
                pos.x() - self.size.x() / 2.,
                pos.y() - self.size.y() / 2.,
                self.size.x(),
                self.size.y(),
            ),
            Color::RED,
        )
        .unwrap();
        draw(ctx, &rect, DrawParam::default())?;
        Ok(())
    }

    pub fn world_to_screen_pos(&self, world_pos: Vec2) -> Vec2 {
        let relative_pos: Vec2 = world_pos - self.pos;
        Vec2::new(
            (relative_pos.x() * BLOCK_SIZE as f32) + self.screen_size.x() / 2.0,
            (relative_pos.y() * BLOCK_SIZE as f32) + self.screen_size.y() / 2.0,
        )
    }
}
