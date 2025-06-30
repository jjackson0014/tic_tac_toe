use crate::prelude::*;
use crate::components::sprite_animation::{AnimationClip, SpriteAnimation};
use crate::resources::animation_library::AnimationLibrary;

// This event is used to trigger an animation on a specific entity
#[derive(Event)]
pub struct TriggerAnimationEvent {
    pub entity: Entity,
    pub clip: AnimationClip,
    pub looping: bool,
    pub frames: usize,
    pub frame_duration: f32,
    pub revert_to: Option<AnimationClip>,
}

// Spawns an animated entity with an idle animation
pub fn spawn_animated_entity_with_idle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    animation_library: &mut ResMut<AnimationLibrary>,
    tag: impl Bundle + Send + Sync + 'static,
    label: &str,
    pos: Vec3,
) {
    let label = label.to_string();
    let sprite_path = format!("sprites/{}_front_idle_anim.png", label);
    let texture: Handle<Image> = asset_server.load(sprite_path);
    animation_library.images.insert((label.clone(), AnimationClip::Idle), texture.clone());

    let texture_atlas_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2 { x: (16), y: (16) }, 4, 1, None, None
    ));

    animation_library.atlas_layouts.insert((label.clone(), AnimationClip::Idle), texture_atlas_layout.clone());

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0, // Assuming the first frame is the idle frame
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(pos.x, pos.y, 2.0)),
        SpriteAnimation {
            frames: 4,
            current_frame: 0,
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            looping: true,
            finished: false,
            next: None,
        },
        tag,
    ));
}

// Loads character animations into the AnimationLibrary
// This function is called to load animations for a character based on the label provided
pub fn load_character_animations(
    animation_library: &mut ResMut<AnimationLibrary>,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    label: &str,
) {
    for (clip, suffix) in [
        (AnimationClip::WalkLeft, "walk_left"),
        (AnimationClip::WalkRight, "walk_right"),
        (AnimationClip::WalkUp, "walk_up"),
        (AnimationClip::WalkDown, "walk_down"),
    ] {
        let label = label.to_string();
        let path = format!("sprites/{}_{}.png", label, suffix);
        let texture: Handle<Image> = asset_server.load(path);
        animation_library.images.insert((label.clone(), clip), texture.clone());

        let texture_atlas_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2 { x: (16), y: (16) }, 4, 1, None, None
        ));
        animation_library.atlas_layouts.insert((label.clone(), clip), texture_atlas_layout);
    }
}

// This system animates sprites based on their SpriteAnimation component
// It loops through the frames of the animation and updates the sprite's texture atlas index
pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut SpriteAnimation)>,
    // mut trigger_writer: EventWriter<TriggerAnimationEvent>,
) {
    for (mut sprite, mut anim) in &mut query {
        if anim.finished {
            continue;
        }
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current_frame += 1;
            if anim.current_frame >= anim.frames {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.finished = true;
                    // If there's a next animation clip, switch to it
                    // [TODO] Right now, it does it just resets but doesnt change
                    // if let Some(next_clip) = &anim.next {
                    //     // anim.finished = false;
                    //     // anim.current_frame = 0;
                    //     // anim.looping = true;
                    // }
                    continue;
                }
            }
            if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                texture_atlas.index = anim.current_frame;
            }
        }
    }
}

// This system transitions to the next animation if the current one is finished
pub fn transition_to_next_animation(
    // mut commands: Commands,
    mut query: Query<(Entity, &mut SpriteAnimation)>,
    mut trigger_writer: EventWriter<TriggerAnimationEvent>,
) {
    for (entity, mut anim) in &mut query {
        if anim.finished {
            if let Some(next_clip) = anim.next.take() {
                anim.finished = false;

                trigger_writer.write(TriggerAnimationEvent {
                    entity,
                    clip: next_clip,
                    looping: true,
                    frames: 4,
                    frame_duration: 0.15,
                    revert_to: None,
                });
            }
        }
    }
}

// This system listens for TriggerAnimationEvent and updates the SpriteAnimation component accordingly
pub fn trigger_animation_event_system(
    // mut commands: Commands,
    mut events: EventReader<TriggerAnimationEvent>,
    mut query: Query<(Entity, &mut SpriteAnimation, &mut Sprite)>,
    animation_library: Res<AnimationLibrary>,
) {
    for event in events.read() {
        if let Ok((_, mut anim, mut sprite)) = query.get_mut(event.entity) {
            let key = ("player".to_string(), event.clip.clone()); // [TODO]: use dynamic label
            if let Some(new_atlas) = animation_library.atlas_layouts.get(&key) {
                if let Some(new_image) = animation_library.images.get(&key) {

                    sprite.image = new_image.clone();

                    if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                        texture_atlas.layout = new_atlas.clone();
                        texture_atlas.index = 0;
                    }

                    *anim = SpriteAnimation {
                        frames: event.frames,
                        current_frame: 0,
                        timer: Timer::from_seconds(event.frame_duration, TimerMode::Repeating),
                        looping: event.looping,
                        finished: false,
                        next: event.revert_to.clone(),
                    };
                }
            }
        } else {
            println!("No animation found for entity: {:?}", event.entity);
        }
    }
}
