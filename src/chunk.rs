use ggez::conf::NumSamples;
use ggez::graphics::{
    draw, drawable_size, get_window_color_format, set_canvas, set_screen_coordinates, Canvas,
    DrawParam,
};
use ggez::{Context, GameResult};
use glam::*;
use std::vec::Vec;

use crate::block::Block;
use crate::config::{BLOCK_SIZE, CHUNK_SIZE};
use crate::resource_manager::ResourceManager;
use crate::utils::Vec2i;
use crate::GameState;

pub struct Chunk {
    pos: Vec2i,
    blocks: Vec<Block>,
    canvas: Canvas,
}

impl Chunk {
    pub fn new(ctx: &mut Context, pos: Vec2i) -> Chunk {
        Chunk {
            pos,
            blocks: Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize),
            canvas: Canvas::new(
                ctx,
                (CHUNK_SIZE * BLOCK_SIZE).try_into().unwrap(),
                (CHUNK_SIZE * BLOCK_SIZE).try_into().unwrap(),
                NumSamples::One,
                get_window_color_format(ctx),
            )
            .unwrap(),
        }
    }

    fn mesh(&mut self, ctx: &mut Context, resource_manager: &ResourceManager) -> GameResult<()> {
        set_canvas(ctx, Some(&self.canvas));
        set_screen_coordinates(
            ctx,
            [
                0.,
                0.,
                (CHUNK_SIZE * BLOCK_SIZE) as f32,
                (CHUNK_SIZE * BLOCK_SIZE) as f32,
            ]
            .into(),
        )?;

        for (i, block) in self.blocks.iter().enumerate() {
            assert!(i as u32 / CHUNK_SIZE < CHUNK_SIZE);
            block.draw(
                ctx,
                Vec2i::new(
                    (i as u32 % CHUNK_SIZE) as i32,
                    (i as u32 / CHUNK_SIZE) as i32,
                ),
                resource_manager,
            )?;
        }

        set_canvas(ctx, None);
        let screen_size = drawable_size(ctx);
        set_screen_coordinates(ctx, [0., 0., screen_size.0, screen_size.1].into())?;
        Ok(())
    }

    pub fn generate(
        &mut self,
        ctx: &mut Context,
        resource_manager: &ResourceManager,
    ) -> GameResult<()> {
        self.blocks
            .resize((CHUNK_SIZE * CHUNK_SIZE) as usize, Block::new(1));
        self.mesh(ctx, resource_manager)
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, ctx: &mut Context, state: &GameState) -> GameResult<()> {
        let x = (self.pos.x * CHUNK_SIZE as i32) as f32;
        let y = (self.pos.y * CHUNK_SIZE as i32) as f32;
        let pos = state.player.world_to_screen_pos((x, y).into());
        draw(
            ctx,
            &self.canvas,
            DrawParam::default().dest(ggez::mint::Point2::from([pos.x(), pos.y()])),
        )?;
        Ok(())
    }
}
