use crate::prelude::*;
use crate::systems::game_debugging::{
    spawn_debug_ui_node,
    update_debug_ui_position,
    update_debug_ui_text,
};
use crate::resources::timers::DebugUiTimer;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                DebugUiTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
            )
            .add_systems(Startup,
                spawn_debug_ui_node,
            )
            .add_systems(Update, (
                update_debug_ui_position,
                update_debug_ui_text,
            ))
            ;
    }
}