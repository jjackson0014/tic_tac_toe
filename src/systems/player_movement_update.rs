use crate::prelude::*;
use crate::resources::user_inputs::GameInput;
use crate::components::player::Player;
use crate::resources::player_config::PlayerPosition;

pub fn handle_player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    mut pos: ResMut<PlayerPosition>,
    input: Res<GameInput>,
) {
    if input.move_delta != IVec2::ZERO {
        pos.x += input.move_delta.x * 16;
        pos.y += input.move_delta.y * 16;

        let mut transform = query.single_mut().expect("Expected a single Player entity");
        transform.translation.x = pos.x as f32;
        transform.translation.y = pos.y as f32;
    }
}

