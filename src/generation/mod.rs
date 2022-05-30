mod noise;

use std::collections::HashMap;

use crate::{
    block::Block,
    chunk::Chunk,
    config::{CHUNK_SIZE, RENDER_DISTANCE},
};

use self::noise::NoiseContext;

pub struct HeightMap {
    pub data: Vec<u32>,
    pub max_height: u32,
}

impl HeightMap {
    pub fn new() -> HeightMap {
        HeightMap {
            data: vec![0; (CHUNK_SIZE * CHUNK_SIZE) as usize],
            max_height: 0,
        }
    }
}

pub struct WorldGenerator {
    height_maps: HashMap<i32, HeightMap>,
    noise_ctx: NoiseContext,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> WorldGenerator {
        WorldGenerator {
            height_maps: HashMap::new(),
            noise_ctx: NoiseContext::new(seed),
        }
    }

    fn get_heigh_map(&mut self, chunk_pos: i32) -> &HeightMap {
        if !self.height_maps.contains_key(&chunk_pos) {
            self.create_height_map(chunk_pos);
        }

        // clean up old height maps
        let mut keys_to_remove: Vec<i32> = vec![];
        for (pos, _height_map) in self.height_maps.iter() {
            if (pos - chunk_pos).abs() > RENDER_DISTANCE * 3 {
                keys_to_remove.push(*pos);
            }
        }
        for key in keys_to_remove {
            self.height_maps.remove(&key);
        }

        self.height_maps.get(&chunk_pos).unwrap()
    }

    fn create_height_map(&mut self, chunk_pos: i32) {
        let mut height_map = HeightMap::new();

        let (data, max_height) = self.noise_ctx.noise(chunk_pos);
        height_map.data = data;
        height_map.max_height = max_height;

        self.height_maps.insert(chunk_pos, height_map);
    }

    pub fn generate(&mut self, chunk: &mut Chunk) {
        let map = self.get_heigh_map(chunk.pos.x);
        if map.max_height as i32 >= chunk.pos.y * CHUNK_SIZE as i32 {
            for i in 0..CHUNK_SIZE {
                let mut height =
                    *map.data.get(i as usize).unwrap() as i32 - (chunk.pos.y * CHUNK_SIZE as i32);
                height = height.clamp(0, CHUNK_SIZE as i32);
                assert!(height >= 0);
                assert!(height <= CHUNK_SIZE as i32);
                let height = height as u32;
                for j in 0..CHUNK_SIZE {
                    chunk.blocks.push(Block::new((j < height) as u32));
                }
            }
        } else {
            chunk
                .blocks
                .resize((CHUNK_SIZE * CHUNK_SIZE).try_into().unwrap(), Block::new(0));
        }
        assert!(chunk.blocks.len() == CHUNK_SIZE as usize * CHUNK_SIZE as usize);
    }
}
