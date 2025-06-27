use crate::prelude::*;
use crate::systems::grid::{spawn_grid};
use crate::resources::grid_config::GridConfig;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GridConfig>()
            .add_systems(Startup,
                spawn_grid,
            );
    }
}