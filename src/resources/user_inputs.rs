use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct GameInput {
    pub move_delta: IVec2,
    pub toggle_scale: bool,
}

impl GameInput {
    pub fn reset(&mut self) {
        self.move_delta = IVec2::ZERO;
        self.toggle_scale = false;
    }
}