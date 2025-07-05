use bevy::prelude::*;

// This resource holds the player's input for movement and actions
#[derive(Resource, Default, Debug)]
pub struct GameInput {
    pub move_delta: IVec2, // The delta movement vector for the player
    pub toggle_scale: bool, // Flag to toggle the scale mode
    pub chosen_map: String,
    pub toggle_debug_node: bool,
}

impl GameInput {
    pub fn reset(&mut self) {
        self.move_delta = IVec2::ZERO;
        self.toggle_scale = false;
        self.chosen_map.clear();
        self.toggle_debug_node = false;
    }
}