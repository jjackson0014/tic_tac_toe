use crate::prelude::*;

// Sprite Animation Component
#[derive(Component)]
pub struct SpriteAnimation {
    pub frames: usize, // Number of frames in the animation
    pub current_frame: usize, // Current frame index
    pub timer: Timer, // Timer to control frame updates
    pub looping: bool, // Whether the animation should loop
    pub finished: bool, // Whether the animation has finished
    pub next: Option<AnimationClip>, // Optional next animation to play
}

// Enum for Animation Clip types
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum AnimationClip {
    Idle,
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
}

