use crate::prelude::*;
use crate::resources::animation_library::AnimationLibrary;
// use crate::components::sprite_animation::{AnimationClip, SpriteAnimation};
use crate::systems::overworld_animation::{
    animate_sprite_system, 
    trigger_animation_event_system,
    TriggerAnimationEvent,
    transition_to_next_animation,
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AnimationLibrary>() // Initialize the AnimationLibrary resource
            .add_event::<TriggerAnimationEvent>() // Add the TriggerAnimationEvent
            .add_systems(Update, (
                animate_sprite_system, // Broader system for animating sprites (i.e. looping through atlas frames)
                transition_to_next_animation, // System to transition to the next animation based on the current state
                trigger_animation_event_system, // System for triggering animation events and (potentially) changing the atlas
            ))
            ;
    }
}