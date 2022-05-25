use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use ggez::filesystem;
use ggez::graphics::Image;
use ggez::{Context, GameResult};

use crate::config::CONFIG_PATH;

pub struct ResourceManager {
    textures: HashMap<u32, Image>,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures: HashMap::new(),
        }
    }

    pub fn load(&mut self, ctx: &mut Context) -> GameResult<()> {
        assert!(
            filesystem::exists(ctx, Path::new(CONFIG_PATH)),
            "Config file not found"
        );
        let mut config_file = filesystem::open(ctx, CONFIG_PATH)?;
        let mut config = String::new();
        config_file.read_to_string(&mut config)?;
        let mut lines = config.lines();
        while let Some(line) = lines.next() {
            let (name, id) = line.rsplit_once("=").expect("Invalid config file");
            let mut name = String::from(name);
            if !name.ends_with(".png") {
                name.push_str(".png");
            }
            if !name.starts_with("/") {
                name.insert_str(0, "/textures/");
            }
            let id = id.parse::<u32>().expect("Invalid config file");
            let img = Image::new(ctx, Path::new(name.as_str())).expect("Invalid config file");
            let r = self.textures.insert(id, img);
            assert!(r.is_none(), "Duplicate texture id");
            println!("Loaded texture: {name} with id {id}");
        }
        println!("Loaded {} textures", self.textures.len());
        Ok(())
    }

    pub fn get(&self, id: u32) -> Option<&Image> {
        self.textures.get(&id)
    }
}
