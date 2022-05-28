mod block;
mod chunk;
mod config;
mod generation;
mod inputs;
mod player;
mod resource_manager;
mod utils;
mod world;

use std::path::Path;
use std::{env, path};

use glam::Vec2;
use inputs::Inputs;
use player::Player;
use resource_manager::ResourceManager;
use world::World;

use ggez::event::*;
use ggez::graphics::Color;
use ggez::graphics::*;
use ggez::*;

pub struct GameState {
    player: Player,
    inputs: Inputs,
    world: World,
    resource_manager: ResourceManager,
}

impl GameState {
    pub fn new(ctx: &mut Context, screen_size: Vec2) -> GameState {
        GameState {
            inputs: Inputs::new(),
            player: Player::new(screen_size),
            world: World::new(),
            resource_manager: ResourceManager::new(ctx),
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player
            .update(&self.inputs, timer::delta(ctx).as_secs_f32());
        self.world
            .update(ctx, &self.player, &self.resource_manager)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::BLACK);

        self.world.draw(ctx, self)?;
        self.player.draw(ctx)?;

        draw(
            ctx,
            &Text::new(timer::fps(ctx).to_string()),
            DrawParam::default(),
        )?;
        draw(
            ctx,
            &Text::new(format!("{}", self.player.pos)),
            DrawParam::default().dest(mint::Vector2 { x: 0., y: 15. }),
        )?;
        present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.inputs.key_down(keycode);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.inputs.key_up(keycode);
    }
}

fn main() {
    let mut cb = ggez::ContextBuilder::new("Platform Master Rust Edition", "Staco")
        .window_setup(conf::WindowSetup::default().title("Platform Master Rust Edition"))
        .resources_dir_name(Path::new("resources").to_str().unwrap())
        .resources_zip_name(Path::new("resources.zip").to_str().unwrap());

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {}", path.display());
        cb = cb.resources_dir_name(path.to_str().unwrap());
    }

    let (mut ctx, event_loop) = cb.build().unwrap();
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);
    let screen_size = Vec2::from(ggez::graphics::drawable_size(&ctx));
    let mut state = GameState::new(&mut ctx, screen_size);
    state.resource_manager.load(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}
