use crate::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    // commands.spawn(Camera2d {
    //     projection: OrthographicProjection {
    //         // Fixed height of virtual screen, width auto-computed by aspect ratio
    //         scaling_mode: ScalingMode::FixedVertical(VIRTUAL_HEIGHT),
    //         ..default()
    //     },
    // ..default()
    // });

    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { 
                viewport_height: (VIRTUAL_HEIGHT) 
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

}


