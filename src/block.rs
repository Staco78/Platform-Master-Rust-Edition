use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::*;
use ggez::GameResult;

use crate::config::{BLOCK_SIZE, CHUNK_SIZE};
use crate::resource_manager::ResourceManager;
use crate::utils::Vec2i;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    id: u32,
}

impl Block {
    pub fn new(id: u32) -> Block {
        Block { id }
    }

    pub fn draw(
        &self,
        pos: Vec2i,
        resource_manager: &ResourceManager,
        batch: &mut SpriteBatch,
    ) -> GameResult<()> {
        let img = resource_manager.get(self.id).unwrap();
        let atlas_width = resource_manager.atlas.width() as f32;
        let atlas_height = resource_manager.atlas.height() as f32;
        assert!(img.w == img.h);
        let scale = BLOCK_SIZE as f32 / img.w as f32;
        let src = Rect::new(
            img.x / atlas_width,
            img.y / atlas_height,
            img.w / atlas_width,
            img.h / atlas_height,
        );
        let params = DrawParam::default()
            .src(src)
            .scale(mint::Vector2 { x: scale, y: scale })
            .dest(ggez::mint::Point2::from([
                (pos.x * BLOCK_SIZE as i32) as f32,
                (CHUNK_SIZE as i32 - pos.y - 1) as f32 * BLOCK_SIZE as f32, // (CHUNK_SIZE as i32 - pos.y - 1) as f32 * BLOCK_SIZE as f32,
            ]));
        batch.add(params);
        Ok(())
    }
}
