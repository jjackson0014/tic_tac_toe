use crate::prelude::*;

#[derive(Resource)]
pub struct GridConfig {
    pub rows: u32,
    pub cols: u32,
    pub tile_size: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig { 
            rows: (VIRTUAL_HEIGHT / 16.0) as u32, 
            cols: (VIRTUAL_WIDTH / 16.0) as u32, 
            tile_size: 16.0 
        }
    }
}