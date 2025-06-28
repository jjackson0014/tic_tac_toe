use crate::prelude::*;
use crate::systems::overworld_startup::{spawn_overworld};
use crate::resources::overworld_config::OverworldConfig;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OverworldConfig>()
            .add_systems(Startup,
                spawn_overworld,
            );
    }
}