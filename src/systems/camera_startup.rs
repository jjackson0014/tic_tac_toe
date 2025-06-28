use crate::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    // 2D Camera
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { 
                viewport_height: (VIRTUAL_SCREEN_HEIGHT) 
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

}


