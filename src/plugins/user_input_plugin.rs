use crate::prelude::*;
use crate::resources::window_config::ScaleMode;
use crate::systems::input_update::*;
use crate::systems::player_movement_update::*;
use crate::systems::window_update::*;
use crate::systems::camera_update::*;
use crate::resources::user_inputs::GameInput;
use crate::resources::player_config::PlayerPosition;
use crate::resources::game_states::GameState;

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameInput>() // Initialize the GameInput resource to track user inputs
            .insert_resource(PlayerPosition { x: 8, y: 8 }) // Initialize player position
            .insert_resource(ScaleMode {multiplier: 1.0}) // Initialize scale mode
            .add_systems(Update,
                collect_game_input, // Collect user inputs each frame
            )
            .add_systems(Update, (
                handle_scale_toggle, // Handle toggling of scale mode
                handle_player_movement // Handle player movement based on inputs
                    .run_if(in_state(GameState::Overworld)), 
                camera_follow // Update camera to follow the player
                    .after(handle_player_movement)
                    .run_if(in_state(GameState::Overworld)),
            ));
    }
}