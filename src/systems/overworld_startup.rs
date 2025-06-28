use crate::prelude::*;
use crate::components::overworld_tile::OverworldTile;
use crate::resources::overworld_config::OverworldConfig;

pub fn spawn_overworld(
    mut commands: Commands,
    overworld_config: Res<OverworldConfig>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Query for our screen window
    let window = windows.single();

    // Set the start of the grid
    let overworld_start_x = -(((overworld_config.cols as f32) * overworld_config.tile_size) / 2.0) + (overworld_config.tile_size / 2.0);
    let overworld_start_y = (((overworld_config.rows as f32) * overworld_config.tile_size) / 2.0) - (overworld_config.tile_size / 2.0);
    println!("Grid starts at: ({}, {})", overworld_start_x, overworld_start_y);

    // Spawn Origin Point (Testing)
    commands.spawn((
        Sprite {
            color: RED,
            custom_size: Some(Vec2::splat(1.0)), 
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    let text_font = TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 4.0,
        ..default()
    };

    // Loop through overworld dimensions
    for row in 0..overworld_config.rows {
        for col in 0..overworld_config.cols {
            let x = overworld_start_x + ((col as f32) * overworld_config.tile_size);
            let y = overworld_start_y - ((row as f32) * overworld_config.tile_size);
            println!("Spawning tile at: ({}, {})", x, y);
                
            // Spawn the tile with its coordinates
            commands.spawn((
                Sprite {
                    color: GRAY,
                    custom_size: Some(Vec2::splat(overworld_config.tile_size - 1.0)), 
                    ..default()
                },
                Transform::from_translation(Vec3::new(x, y, 0.0)),
                OverworldTile {
                    tile_world_pixel_coordinates: Vec2::new(x,y),
                    tile_world_grid_coordinates: Vec2::new(col as f32, row as f32),
                },
            ))
            .with_children( |builder| {
                builder.spawn((
                    Text2d::new(format!("({},{})",col,row)),
                    text_font.clone(),
                    TextColor(RED.into()),
                    TextLayout::new(JustifyText::Center, LineBreak::AnyCharacter),
                    // Ensure the text is drawn on top of the box
                    Transform::from_translation(Vec3::Z),
                ));
            })
            ;
        }
    }
}