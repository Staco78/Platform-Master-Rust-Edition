pub const CHUNK_SIZE: u32 = 16;
pub const PLAYER_SPEED: f32 = 30.0;
pub const BLOCK_SIZE: u32 = 32;
pub const RENDER_DISTANCE: i32 = 4;
pub const ATLAS_PATH: &str = "/atlas.png";
pub const ATLAS_CONFIG_PATH: &str = "/atlasConfig.bin";

pub enum DebugMode {
    ChunkBorder,
}

pub fn debug_feature_enabled(_mode: DebugMode) -> bool {
    #[cfg(not(debug_assertions))]
    return false;

    #[cfg(debug_assertions)]
    match _mode {
        DebugMode::ChunkBorder => true,
        #[allow(unreachable_patterns)]
        _ => false,
    }
}
