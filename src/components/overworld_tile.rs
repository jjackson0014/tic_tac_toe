use crate::prelude::*;
use bevy::{color::Srgba, math::Vec2};

#[derive(Component)]
pub struct OverworldTile {
    pub tile_world_pixel_coordinates: Vec2,
    pub tile_world_grid_coordinates: Vec2,
}

// Component for Tile Text
#[derive(Component)]
pub struct OverworldTileText {
    // pub text: String,
}