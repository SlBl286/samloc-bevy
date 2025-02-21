use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(Timer);
