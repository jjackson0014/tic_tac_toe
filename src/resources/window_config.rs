use crate::prelude::*;

#[derive(Resource)]
pub struct ScaleMode {
  pub multiplier: f32, // 1.0 for 240×160, 2.0 for 480×320
}