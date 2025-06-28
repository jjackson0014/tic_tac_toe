use bevy::prelude::*;

// This resource holds the player's input for movement and actions
#[derive(Resource, Default, Debug)]
pub struct GameInput {
    pub move_delta: IVec2, // The delta movement vector for the player
    pub toggle_scale: bool, // Flag to toggle the scale mode
}

impl GameInput {
    pub fn reset(&mut self) {
        self.move_delta = IVec2::ZERO;
        self.toggle_scale = false;
    }
}