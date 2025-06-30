use crate::prelude::*;

// Component for Overworld Tile
#[derive(Component)]
pub struct OverworldTile {
    pub tile_world_pixel_coordinates: IVec2,
    //pub tile_world_grid_coordinates: Vec2,
}

// Component: Tile Text/Caption
#[derive(Component)]
pub struct OverworldTileText {}

// Component: Tile Type
// Type of tile in the overworld
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverworldTileType {
    Ground,
    Wall,
    Water,
    Void,
}

// 
impl OverworldTileType {

    // Returns the color associated with the tile type
    pub fn tile_color(self) -> Color {
        match self {
            OverworldTileType::Ground => GRAY,
            OverworldTileType::Wall => RED,
            OverworldTileType::Water => BLUE,
            OverworldTileType::Void => BLACK,
        }
    }

    
    pub fn is_walkable(self) -> bool {
        matches!(self, OverworldTileType::Ground)
    }
}
