use crate::prelude::*;
use crate::resources::window_config::ScaleMode;
use crate::systems::input_update::*;
use crate::systems::player_movement_update::*;
use crate::systems::window_update::*;
use crate::resources::user_inputs::GameInput;
use crate::resources::player_config::PlayerPosition;

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameInput>()
            .insert_resource(PlayerPosition { x: 8, y: 8 }) // Initialize player position
            .insert_resource(ScaleMode {multiplier: 1.0}) // Initialize scale mode
            .add_systems(Update,
                collect_game_input,
            )
            .add_systems(Update, (
                handle_player_movement,
                handle_scale_toggle,
            ));
    }
}