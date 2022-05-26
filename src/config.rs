pub const CHUNK_SIZE: u32 = 32;
pub const PLAYER_SPEED: f32 = 10.0;
pub const BLOCK_SIZE: u32 = 30;
pub const RENDER_DISTANCE: i32 = 2;
pub const CONFIG_PATH: &str = "/config.txt";

#[cfg(debug_assertions)]
pub enum DebugMode {
    ChunkBorder,
}

#[cfg(debug_assertions)]
pub fn feature_enabled(mode: DebugMode) -> bool {
    match mode {
        DebugMode::ChunkBorder => true,
        #[allow(unreachable_patterns)]
        _ => false,
    }
}
