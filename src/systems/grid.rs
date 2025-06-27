use crate::prelude::*;
use crate::components::grid_tile::GridTile;
use crate::resources::grid_config::GridConfig;

pub fn spawn_grid(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Query for our screen window
    let window = windows.single();

    // Set the start of the grid
    let start_x = -((grid_config.cols as f32) * grid_config.tile_size) / 2.0;
    let start_y = -((grid_config.rows as f32) * grid_config.tile_size) / 2.0;

    for row in 0..grid_config.rows {
        for col in 0..grid_config.cols {
            let x = start_x + (col as f32) * grid_config.tile_size;
            let y = start_y + (row as f32) * grid_config.tile_size;

            commands.spawn((
                Sprite {
                    color: WHITE,
                    custom_size: Some(Vec2::splat(grid_config.tile_size - 2.0)), 
                    ..default()
                },
                Transform::from_translation(Vec3::new(x, y, 0.0)),
                GridTile {
                    grid_position: Vec2::new(col as f32, row as f32),
                },
            ));
        }
    }
}