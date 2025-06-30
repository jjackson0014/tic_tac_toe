use crate::prelude::*;
use crate::components::{
    overworld_tile::{OverworldTile, OverworldTileType},
    marker::Player,
    clearable_ui::ClearableUI
};
use crate::resources::{
    overworld_config::OverworldConfig,
    game_states::GameState,
    map_load_request::LoadedMap,
    map_load_request::MapLoadState,
    map_load_request::MapHasLoaded,
    animation_library::AnimationLibrary,
};
use crate::systems::{
    overworld_animation::spawn_animated_entity_with_idle,
    overworld_animation::load_character_animations,
};
use crate::plugins::{
    TileMapAsset
};

// This system starts the map loading process by requesting the map asset from the AssetServer
// It uses the LoadedMap resource to determine which map to load and updates the MapLoadState with the handle to the map asset.
pub fn start_map_load(
    mut map_load_state: ResMut<MapLoadState>,
    asset_server: Res<AssetServer>,
    loaded_map: Res<LoadedMap>,
) {
    // println!("Started loading map: {}", format!("maps\\{}.json", loaded_map.0));
    let handle = asset_server.load(format!("maps\\{}.json", loaded_map.0)); // Load the map asset from the AssetServer
    // println!("Map load handle: {:?}", handle);
    map_load_state.handle = Some(handle); // Store the handle in the MapLoadState resource
}

// Despawns all tiles and the player when changing maps
pub fn despawn_overworld(
    mut commands: Commands,
    tiles: Query<Entity, With<OverworldTile>>,
    players: Query<Entity, With<Player>>,
    texts: Query<Entity, With<Text2d>>,
    ui_texts: Query<Entity, With<ClearableUI>>,
) {
    for entity in &tiles {
        commands.entity(entity).despawn();
    }
    for player in &players {
        commands.entity(player).despawn();
    }
    for text in &texts {
        commands.entity(text).despawn();
    }
    for ui_text in &ui_texts {
        commands.entity(ui_text).despawn();
    }
}

// Clear the UI text that is meant to be cleared when the map is loaded
pub fn despawn_clearable_ui(
    mut commands: Commands,
    loading_texts: Query<Entity, With<ClearableUI>>,
) {
    for ui_text in &loading_texts {
        commands.entity(ui_text).despawn();
    }
}

// Shows a 'Loading...' text in the center of the screen while the map is loading
pub fn set_loading_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // println!("Setting loading UI...");
    
    // Load the font asset
    let text_font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        ..default()
    };

    // Spawn the loading text
    commands.spawn((
                    Text2d::new("Loading..."),
                    text_font.clone(),
                    TextColor(WHITE.into()),
                    TextLayout::new(JustifyText::Center, LineBreak::AnyCharacter),
                    // Ensure the text is drawn on top of the box
                    Transform::from_translation(Vec3::Z),
                    ClearableUI,
    ));
}

// Resets the map loaded flag to false, allowing the map to be loaded again
pub fn reset_map_flags(mut has_loaded: ResMut<MapHasLoaded>) {
    has_loaded.0 = false;
}

// This system spawns the overworld map from a JSON file, creating tiles and a player entity based on the map data.
pub fn spawn_overworld_from_json(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemaps: Res<Assets<TileMapAsset>>,
    // current_map: Res<LoadedMap>,
    map_load_state: Res<MapLoadState>,
    overworld_config: Res<OverworldConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animation_library: ResMut<AnimationLibrary>,
    mut map_has_loaded: ResMut<MapHasLoaded>,
) {
    // Check if the map has already loaded
     if map_has_loaded.0 {
        return;
    }

    // Check if the map handle is available
    let Some(handle) = &map_load_state.handle else {
        println!("No map handle found!");
        return;
    };
    
    // Check if the map is still loading
    // let load_state = asset_server.get_load_state(handle);
    // println!("Map is still loading... Handle... {:?} Current state: {:?}", handle, load_state);
    
    if let Some(map_asset) = tilemaps.get(handle) {

        // println!("Map loaded successfully: {}", current_map.0);
        
        let tile_size = overworld_config.tile_size;
        let num_rows = map_asset.tiles.len();
        let num_cols = map_asset.tiles.first().map(|r| r.len()).unwrap_or(0);

        let start_x = -((num_cols as f32) * tile_size) / 2.0 + tile_size / 2.0;
        let start_y = ((num_rows as f32) * tile_size) / 2.0 - tile_size / 2.0;

        // let mut player_spawned = false;

        for (row, cols) in map_asset.tiles.iter().enumerate() {
            for (col, symbol) in cols.iter().enumerate() {
                let tile_type = match symbol.as_str() {
                    "." => OverworldTileType::Ground,
                    "P" => OverworldTileType::Ground, // Player spawn point
                    "~" => OverworldTileType::Water,
                    "X" => OverworldTileType::Wall,
                    "x" => OverworldTileType::Void,
                    _ => continue,
                };

                let x = start_x + col as f32 * tile_size;
                let y = start_y - row as f32 * tile_size;

                // spawn player at "P"
                if symbol == "P" {
                    // commands.spawn((
                    //     Sprite::from_image(asset_server.load("sprites/player_front_still_ss.png")),
                    //     Transform::from_translation(Vec3::new(x, y, 2.0)),
                    //     Player,
                    // ));

                    spawn_animated_entity_with_idle(
                        &mut commands,
                        &asset_server,
                        &mut atlas_layouts,
                        &mut animation_library,
                        Player,
                        "player",
                        Vec3::new(x,y,2.0),
                    );

                    load_character_animations(
                        &mut animation_library, 
                        &asset_server, 
                        &mut atlas_layouts, 
                        "player"
                    );

                    // player_spawned = true;
                }

                // let text_font = TextFont {
                //     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                //     font_size: 4.0,
                //     ..default()
                // };

                // Spawn the tile with its SYMBOL
                commands.spawn((
                    Sprite {
                        color: tile_type.tile_color(),
                        custom_size: Some(Vec2::splat(overworld_config.tile_size - 1.0)), 
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(x, y, 0.0)),
                    OverworldTile {
                        tile_world_pixel_coordinates: IVec2::new(x as i32, y as i32),
                        //tile_world_grid_coordinates: Vec2::new(col as f32, row as f32),
                    },
                    tile_type,
                ));
                // .with_children( |builder| {
                //     builder.spawn((
                //         Text2d::new(format!("({})",symbol)),
                //         text_font.clone(),
                //         TextColor(BLACK.into()),
                //         TextLayout::new(JustifyText::Center, LineBreak::AnyCharacter),
                //         // Ensure the text is drawn on top of the box
                //         Transform::from_translation(Vec3::Z),
                //     ));
                // });
            }
        }

        map_has_loaded.0 = true; // Set map loaded flag to true to prevent reloading
        next_state.set(GameState::Overworld); // Change the game state to Overworld after loading the map
    }
}