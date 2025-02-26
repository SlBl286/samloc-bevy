use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SliderDelayMoveTimer(Timer);

impl Default for SliderDelayMoveTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
}