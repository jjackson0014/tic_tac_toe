use crate::prelude::*;
use crate::components::sprite_animation::{AnimationClip};

// The AnimationLibrary resource holds the animation atlas layouts and images for different animation clips
#[derive(Resource, Default)]
pub struct AnimationLibrary {
    pub atlas_layouts: HashMap<(String, AnimationClip), Handle<TextureAtlasLayout>>, 
    pub images: HashMap<(String, AnimationClip), Handle<Image>>,
}

