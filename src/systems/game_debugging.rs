// game_state_debug.rs
use crate::prelude::*;
use crate::resources::{
    game_states::GameState,
    user_inputs::GameInput,
    window_config::ScaleMode,
    map_load_request::LoadedMap,
    timers::DebugUiTimer,
};
use crate::components::{
    marker::DebugUI,
    marker::DebugText,
};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    window::WindowResized,
};

// Prints a message whenever the game state changes
// pub fn debug_game_state(state: Res<State<GameState>>) {
//     if state.is_changed() {
//         println!("Game state changed to: {:?}", state.get());
//     }
// }

pub fn handle_scale_toggle(
    input: Res<GameInput>,
    mut mode: ResMut<ScaleMode>,
    mut camera_query: Query<&mut Projection, With<Camera>>,
    mut ui_scale: ResMut<UiScale>,
) {
    if input.toggle_scale {
        mode.multiplier = if (mode.multiplier - 1.0).abs() < f32::EPSILON { 2.0 } else { 1.0 };

        if let Ok(mut proj) = camera_query.single_mut() {
            if let Projection::Orthographic(ref mut ortho) = *proj {
                ortho.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical {viewport_height: (VIRTUAL_SCREEN_HEIGHT * mode.multiplier)};
            }
        }

        ui_scale.0 = mode.multiplier

    }
}

pub fn handle_debug_map_load(
    input: Res<GameInput>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_map: ResMut<LoadedMap>,
) {
    if input.chosen_map == "testmapS" {
        *current_map = LoadedMap("testmapS".to_string());
        next_state.set(GameState::Loading);
    } else if input.chosen_map == "testmapM" {
        *current_map = LoadedMap("testmapM".to_string());
        next_state.set(GameState::Loading);
    } else if input.chosen_map == "testmapL" {
        *current_map = LoadedMap("testmapL".to_string());
        next_state.set(GameState::Loading);
    }
}

pub fn handle_debug_ui_node(
    input: Res<GameInput>,
    mut debug_ui_query: Query<&mut Node, With<DebugUI>>,
) {
    if input.toggle_debug_node {
        for mut node in debug_ui_query.iter_mut() {
            if node.display == Display::None {
                node.display = Display::Flex; // Show the debug UI
            } else {
                node.display = Display::None; // Hide the debug UI
            }
        }
    }
}

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
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        BackgroundColor(BLACK.into()),
        BorderColor(WHITE.into()),
        DebugUI, // Custom component to mark this as a debug UI node
    ))
    
    .with_children(|builder| {
        builder.spawn((
            text_font.clone(),
            TextColor(WHITE.into()),
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

pub fn update_debug_ui_text(
    diagnostics: Res<DiagnosticsStore>,
    game_state: Res<State<GameState>>,
    user_input: Res<GameInput>,
    mut query: Query<&mut Text, With<DebugText>>,
    mut timer: ResMut<DebugUiTimer>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {} | GameState: {:?} | UserInput: {:?}", value.round(), game_state.get(), user_input)
            }
        }
    }
}