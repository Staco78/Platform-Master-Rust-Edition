use ggez::{Context, GameResult};
use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    config::{CHUNK_SIZE, RENDER_DISTANCE},
    generation::WorldGenerator,
    player::Player,
    resource_manager::ResourceManager,
    utils::Vec2i,
    GameState,
};

pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    generator: WorldGenerator,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::with_capacity(50),
            generator: WorldGenerator::new(1),
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        player: &Player,
        resource_manager: &ResourceManager,
    ) -> GameResult<()> {
        let chunk_pos = Vec2i::new(
            (player.pos.x() / CHUNK_SIZE as f32) as i32,
            (player.pos.y() / CHUNK_SIZE as f32) as i32,
        );
        for x in (chunk_pos.x - RENDER_DISTANCE)..=(chunk_pos.x + RENDER_DISTANCE) {
            for y in (chunk_pos.y - RENDER_DISTANCE)..=(chunk_pos.y + RENDER_DISTANCE) {
                if !self.chunks.contains_key(&(x, y)) {
                    self.chunks
                        .insert((x, y), Chunk::new(ctx, Vec2i::new(x, y)));
                    self.chunks.get_mut(&(x, y)).unwrap().generate(
                        ctx,
                        resource_manager,
                        &mut self.generator,
                    )?;
                }
                self.chunks.get_mut(&(x, y)).unwrap().update();
            }
        }
        // TODO destroy chunks that are out of range

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context, state: &GameState) -> GameResult<()> {
        for (_pos, chunk) in self.chunks.iter() {
            chunk.draw(ctx, state)?;
        }
        Ok(())
    }
}
