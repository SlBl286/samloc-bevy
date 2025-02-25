use bevy::prelude::*;

#[derive(Event)]
pub struct SliderMoveEvent(pub f32);