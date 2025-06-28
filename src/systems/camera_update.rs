use crate::prelude::*;
use crate::components::player::Player;

// This system updates the camera's position to follow the player entity
// It assumes there is only one Player and one Camera entity in the scene
pub fn camera_follow (
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single().expect("Expected a single Player entity");
    let mut cam_transform = cam_query.single_mut().expect("Expected a single Camera entity");

    cam_transform.translation.x = player_transform.translation.x;
    cam_transform.translation.y = player_transform.translation.y;
}