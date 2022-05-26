use std::collections::HashMap;

use crate::{
    chunk::Chunk,
    config::{CHUNK_SIZE, RENDER_DISTANCE},
};

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
    seed: u64,
    height_maps: HashMap<i32, HeightMap>,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> WorldGenerator {
        WorldGenerator {
            seed,
            height_maps: HashMap::new(),
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

    fn create_height_map(&mut self, chunk_pos: i32) -> &HeightMap {
        let mut height_map = HeightMap::new();
        for i in 0..CHUNK_SIZE {
            *height_map.data.get_mut(i as usize).unwrap() = 50;
        }
        height_map.max_height = 50;
        self.height_maps.insert(chunk_pos, height_map);
        self.height_maps.get(&chunk_pos).unwrap()
    }

    pub fn generate(&mut self, chunk: &mut Chunk) {
        let map = self.get_heigh_map(chunk.pos.x);
        if map.max_height as i32 >= chunk.pos.y * CHUNK_SIZE as i32 {
            for i in 0..CHUNK_SIZE {
                let mut height =
                    *map.data.get(i as usize).unwrap() as i32 - (chunk.pos.y * CHUNK_SIZE as i32);
                if height > CHUNK_SIZE as i32 {
                    height = CHUNK_SIZE as i32;
                }
                assert!(height >= 0);
                assert!(height <= CHUNK_SIZE as i32);
                let height = height as u32;
                for j in 0..height {
                    chunk.set_block(i, j, 1);
                }
            }
        }
    }
}
