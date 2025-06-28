use crate::prelude::*;

#[derive(Resource)]
pub struct OverworldConfig {
    pub rows: u32,
    pub cols: u32,
    pub tile_size: f32,
}

impl Default for OverworldConfig {
    fn default() -> Self {
        OverworldConfig { 
            rows: ((VIRTUAL_SCREEN_HEIGHT * 4.0) / 16.0) as u32, 
            cols: ((VIRTUAL_SCREEN_WIDTH  * 4.0) / 16.0) as u32, 
            tile_size: 16.0 
        }
    }
}