use crate::prelude::*;

// This system sets up the camera for the game - Right now it is just a default 2D camera with fixed vertical scaling
pub fn camera_setup(mut commands: Commands) {
    // Constants for the virtual screen dimensions
    let height = VIRTUAL_SCREEN_HEIGHT;
    let width = VIRTUAL_SCREEN_WIDTH;
    
    // 2D Camera
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { 
                viewport_height: height
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(width / 2.0, -height / 2.0, 0.0) // Center the camera in the virtual screen
    ));
}
