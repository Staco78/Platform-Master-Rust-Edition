use ggez::conf::NumSamples;
use ggez::graphics::{
    draw, drawable_size, get_window_color_format, set_canvas, set_screen_coordinates, Canvas,
    Color, DrawMode, DrawParam, Mesh, Rect,
};
use ggez::{Context, GameResult};
use glam::*;
use std::vec::Vec;

use crate::block::Block;
use crate::config::{BLOCK_SIZE, CHUNK_SIZE};
use crate::generation::WorldGenerator;
use crate::resource_manager::ResourceManager;
use crate::utils::Vec2i;
use crate::GameState;

pub struct Chunk {
    pub pos: Vec2i,
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

        #[cfg(debug_assertions)]
        {
            if crate::config::feature_enabled(crate::config::DebugMode::ChunkBorder) {
                let rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::stroke(1.),
                    Rect::new(
                        0.,
                        0.,
                        (CHUNK_SIZE * BLOCK_SIZE) as f32,
                        (CHUNK_SIZE * BLOCK_SIZE) as f32,
                    ),
                    Color::RED,
                )?;
                draw(ctx, &rect, DrawParam::default())?;
            }
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
        generator: &mut WorldGenerator,
    ) -> GameResult<()> {
        self.blocks
            .resize((CHUNK_SIZE * CHUNK_SIZE) as usize, Block::new(0));
        generator.generate(self);
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
            DrawParam::default()
                .dest(ggez::mint::Point2::from([pos.x(), pos.y()]))
                .offset(ggez::mint::Vector2 {
                    x: 0.,
                    y: BLOCK_SIZE as f32 * CHUNK_SIZE as f32,
                }),
        )?;
        Ok(())
    }

    pub fn set_block(&mut self, x: u32, y: u32, id: u32) {
        self.blocks[(x + y * CHUNK_SIZE) as usize] = Block::new(id);
    }
}
