use crate::prelude::*;
use crate::resources::{
    overworld_config::OverworldConfig,
    game_states::GameState,
    map_load_request::LoadedMap,
    map_load_request::MapLoadState,
    map_load_request::MapHasLoaded,
};
use crate::systems::overworld_setup::{
    set_loading_ui,
    spawn_overworld_from_json,
    despawn_overworld,
    despawn_clearable_ui,
    start_map_load,
    reset_map_flags,
};
use crate::plugins::json_map_loader_plugin::{TileMapAsset, TileMapLoader};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OverworldConfig>() // Initialize the OverworldConfig resource
            .init_asset::<TileMapAsset>() // Initialize the TileMapAsset type
            .init_asset_loader::<TileMapLoader>() // Register the custom asset loader for TileMapAsset
            .init_resource::<MapLoadState>() // Initialize the MapLoadState resource
            .init_resource::<MapHasLoaded>() // Initialize the MapHasLoaded resource
            .init_resource::<LoadedMap>() // Initialize the CurrentMap resource
            // Enter Loading Screen
            .add_systems(OnEnter(GameState::Loading), (
                reset_map_flags, // Reset map loading flags when entering the Loading state
                despawn_overworld, // Despawn any existing overworld entities
                set_loading_ui, // Set up the loading UI when entering the Loading state
                start_map_load, // Start the map loading process
                despawn_clearable_ui, // Clear any UI elements that can be reset
            ).chain()) // Chain the systems to run in sequence when entering the Loading state
            .add_systems(Update, (
                //debug_game_state, // Debugging system to log the current game state
                spawn_overworld_from_json.run_if(in_state(GameState::Loading)), // Spawn the overworld from JSON when in the Loading state
            ));
    }
}