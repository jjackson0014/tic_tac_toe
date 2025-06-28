use crate::prelude::*;

// Resource storing the player's current grid position
#[derive(Resource, Default, Debug)]
pub struct PlayerPosition {
    pub x: i32,
    pub y: i32,
}