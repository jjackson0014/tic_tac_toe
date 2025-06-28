use crate::prelude::*;
use crate::resources::user_inputs::GameInput;
use crate::resources::window_config::{ScaleMode};

pub fn handle_scale_toggle(
    input: Res<GameInput>,
    mut mode: ResMut<ScaleMode>,
    mut camera_query: Query<&mut Projection, With<Camera>>,
) {
    if input.toggle_scale {
        mode.multiplier = if (mode.multiplier - 1.0).abs() < f32::EPSILON {
            2.0
        } else {
            1.0
        };

        if let Ok(mut proj) = camera_query.single_mut() {
            if let Projection::Orthographic(ref mut ortho) = *proj {
                //ortho.scaling_mode = ScalingMode::FixedVertical(VIRTUAL_SCREEN_HEIGHT * mode.multiplier);
                ortho.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical {viewport_height: (VIRTUAL_SCREEN_HEIGHT * mode.multiplier)};
            }
        }
    }
}