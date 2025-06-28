// Prelude:
// Crates and Constants to be used thoughout the app.
mod prelude {
    pub use bevy::prelude::*; // 0.16

    // Widndow
    pub const VIRTUAL_SCREEN_WIDTH: f32 = 240.0;
    pub const VIRTUAL_SCREEN_HEIGHT: f32 = 160.0;

    // Colors
    pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
    pub const PINK: Color = Color::srgb(1.0, 0.75,0.80);
    pub const GRAY: Color = Color::srgb(0.75, 0.75,0.75);
    pub const TEAL: Color = Color::srgb(0.39, 1.0, 1.0);
    pub const ORANGE: Color = Color::srgb(0.90, 0.75, 0.16);
}
mod components;
mod resources;
mod systems;
mod plugins;

use prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (VIRTUAL_SCREEN_WIDTH, VIRTUAL_SCREEN_HEIGHT).into(),
                        title: "Basic Window".into(),
                        ..default()
                    }),
                    ..default()
                }
            )
        )
        .add_plugins((
            plugins::OverworldPlugin,
        ))
        .add_systems(
            Startup, 
            (
                systems::camera_startup::camera_setup,
            )
        )
        .add_plugins((
            plugins::UserInputPlugin,
        ))
        .run();
}