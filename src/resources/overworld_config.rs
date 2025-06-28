use crate::prelude::*;

// This resource holds configuration settings for the overworld
#[derive(Resource)]
pub struct OverworldConfig {
    pub tile_size: f32,
}

impl Default for OverworldConfig {
    fn default() -> Self {
        OverworldConfig { 
            tile_size: TILE_SIZE, 
        }
    }
}