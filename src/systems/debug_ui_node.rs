use crate::prelude::*;
use crate::components::marker::{DebugUI};
use bevy::window::WindowResized;


pub fn spawn_debug_ui_node(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 10.0,
        ..default()
    };

    commands.spawn((
        Node {
            display: Display::None, // Default as hidden
            width: Val::Percent(100.0),
            height: Val::Px(DEBUG_UI_HEIGHT), // Height for the debug UI
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0), // Position it at the bottom of the screen
            left: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(RED.into()),
        DebugUI, // Custom component to mark this as a debug UI node
    ))
    .with_children(|builder| {
        builder.spawn((
            text_font.clone(),
            TextColor(BLACK.into()),
            Text::new(""),
            DebugText,
        ));
    });
}

pub fn update_debug_ui_position(
    resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Node, With<DebugUI>>,
) {
    if resize_reader.is_empty() {
        return; // No resize event, nothing to update
    }

    let y_offset_in_pixels = 0.0;

    for mut node in &mut query {
        node.top = Val::Auto;
        node.bottom = Val::Px(y_offset_in_pixels);
    }
}

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use crate::components::marker::DebugText;
use crate::GameState;


pub fn update_debug_ui_text(
    diagnostics: Res<DiagnosticsStore>,
    game_state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            //println!("We have FPS");
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {} | GameState: {:?}", value, game_state.get())
            }
        }
    }
}