use ggez::graphics::{Image, Rect};
use ggez::{filesystem, Context, GameResult};

use std::io::Read;
use std::path::Path;
use std::vec::Vec;

use crate::config::{ATLAS_CONFIG_PATH, ATLAS_PATH};

pub struct ResourceManager {
    pub atlas: Image,
    textures: Vec<Rect>,
}

impl ResourceManager {
    pub fn new(ctx: &mut Context) -> ResourceManager {
        ResourceManager {
            atlas: Image::new(ctx, Path::new(ATLAS_PATH)).expect("atlas.png not found"),
            textures: Vec::new(),
        }
    }

    pub fn load(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut file = filesystem::open(ctx, Path::new(ATLAS_CONFIG_PATH))?;
        let mut config = Vec::new();
        file.read_to_end(&mut config)?;
        let img_count = config.len() / 16;
        assert!(config.len() % 16 == 0);
        for i in 0..img_count {
            let x = u32::from_le_bytes(config[i * 16..i * 16 + 4].try_into().unwrap());
            let y = u32::from_le_bytes(config[i * 16 + 4..i * 16 + 8].try_into().unwrap());
            let w = u32::from_le_bytes(config[i * 16 + 8..i * 16 + 12].try_into().unwrap());
            let h = u32::from_le_bytes(config[i * 16 + 12..i * 16 + 16].try_into().unwrap());

            println!("load texture {i} at pos ({x}, {y})");

            self.textures
                .push(Rect::new_i32(x as i32, y as i32, w as i32, h as i32));
        }
        Ok(())
    }

    pub fn get(&self, id: u32) -> Option<&Rect> {
        self.textures.get(id as usize)
    }

}
