use crate::prelude::*;
use crate::resources::user_inputs::GameInput;
use crate::components::marker::Player;
use crate::resources::player_config::PlayerPosition;
use crate::components::overworld_tile::{OverworldTile, OverworldTileType};
use crate::components::sprite_animation::{AnimationClip};
use crate::systems::overworld_animation::TriggerAnimationEvent;

pub fn handle_player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    mut pos: ResMut<PlayerPosition>,
    input: Res<GameInput>,
    tiles: Query<(&OverworldTile, &OverworldTileType)>,
    mut trigger_writer: EventWriter<TriggerAnimationEvent>,
    query_entity: Query<Entity, With<Player>>,
) {
    if input.move_delta != IVec2::ZERO {
        let target_pos = IVec2::new(pos.x + (input.move_delta.x * 16), pos.y + (input.move_delta.y * 16));

        let maybe_tile = tiles.iter()
            .find(|(tile, _)| tile.tile_world_pixel_coordinates == target_pos);

        if let Some((_, tile_type)) = maybe_tile {
            if tile_type.is_walkable() {
                pos.x = target_pos.x;
                pos.y = target_pos.y;

                // Animation logic based on movement direction
                let clip = if input.move_delta.x < 0 {
                    AnimationClip::WalkLeft
                } else if input.move_delta.x > 0 {
                    AnimationClip::WalkRight
                } else if input.move_delta.y > 0 {
                    AnimationClip::WalkUp
                } else if input.move_delta.y < 0 {
                    AnimationClip::WalkDown
                } else {
                    AnimationClip::Idle
                };

                // println!("Clip: {:?}", clip); 

                if let Ok(entity) = query_entity.single() {
                    trigger_writer.write(TriggerAnimationEvent {
                        entity,
                        clip,
                        looping: false,
                        frames: 4,
                        frame_duration: 0.1,
                        revert_to: Some(AnimationClip::Idle),
                    });

                    // println!("Triggered animation event for entity: {:?}", entity);
                }

                // Actual movement of the player entity
                let mut transform = query.single_mut().expect("Expected a single Player entity");
                transform.translation.x = pos.x as f32;
                transform.translation.y = pos.y as f32;

            } else {
                println!("Blocked: Tile at {:?} is {:?}", target_pos, tile_type);
            }
        } else {
            println!("No tile found at {:?}", target_pos);
        }
    }
}

