use crate::prelude::*;
use crate::systems::debug_ui_node::{
    spawn_debug_ui_node,
    update_debug_ui_position,
    update_debug_ui_text,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
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