// Prelude:
// Crates and Constants to be used thoughout the app.
mod prelude {
    pub use bevy::prelude::*; // 0.16
    pub use serde::{Deserialize,}; // 1.0
    pub use anyhow::Result; // 1.0
    pub use std::collections::HashMap;

    // Window
    pub const VIRTUAL_SCREEN_WIDTH: f32 = 240.0;
    pub const VIRTUAL_SCREEN_HEIGHT: f32 = 160.0;
    pub const DEBUG_UI_HEIGHT: f32 = 40.0;

    // Colors
    
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    pub const GRAY: Color = Color::srgb(0.75, 0.75,0.75);
    pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
    //pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
    //pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
    //pub const PINK: Color = Color::srgb(1.0, 0.75,0.80);
    //pub const TEAL: Color = Color::srgb(0.39, 1.0, 1.0);
    //pub const ORANGE: Color = Color::srgb(0.90, 0.75, 0.16);

    // Overworld
    pub const TILE_SIZE: f32 = 16.0; // Size of each tile
}
mod components;
mod resources;
mod systems;
mod plugins;

use prelude::*;
use crate::resources::game_states::GameState;
use bevy_diagnostic::FrameTimeDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            resolution: (VIRTUAL_SCREEN_WIDTH, VIRTUAL_SCREEN_HEIGHT + DEBUG_UI_HEIGHT).into(),
                            resizable: false,
                            title: "New Game".into(),
                            ..default()
                        }),
                        ..default()
                    },
                )
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<GameState>()
        .add_plugins((
            plugins::OverworldPlugin,
            plugins::UserInputPlugin,
            plugins::AnimationPlugin,
            plugins::UIPlugin,
        ))
        .add_systems(
            Startup, 
            (
                systems::camera_startup::camera_setup,
            )
        )
        .run();
}