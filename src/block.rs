use ggez::graphics::*;
use ggez::{Context, GameResult};

use crate::config::{BLOCK_SIZE, CHUNK_SIZE};
use crate::resource_manager::ResourceManager;
use crate::utils::Vec2i;

#[derive(Clone, Copy)]
pub struct Block {
    id: u32,
}

impl Block {
    pub fn new(id: u32) -> Block {
        Block { id }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        pos: Vec2i,
        resource_manager: &ResourceManager,
    ) -> GameResult<()> {
        let img = resource_manager.get(self.id).unwrap();
        let scale = BLOCK_SIZE as f32 / img.width() as f32;
        draw(
            ctx,
            img,
            DrawParam::default()
                .dest(ggez::mint::Point2::from([
                    (pos.x * BLOCK_SIZE as i32) as f32,
                    ((CHUNK_SIZE as i32 - pos.y - 1) * BLOCK_SIZE as i32) as f32,
                ]))
                .scale(mint::Vector2 { x: scale, y: scale }),
        )?;
        Ok(())
    }
}
