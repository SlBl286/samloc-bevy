use bevy::prelude::*;
use components::OnLoadingScreen;
use samloc::despawn_screen;
use systems::{countdown, loading_setup};

use crate::states::AppStates;

pub mod components;
pub mod resources;
mod systems;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::LoadingScreen), loading_setup)
            .add_systems(Update, countdown.run_if(in_state(AppStates::LoadingScreen)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(AppStates::LoadingScreen), despawn_screen::<OnLoadingScreen>);
    }
}
