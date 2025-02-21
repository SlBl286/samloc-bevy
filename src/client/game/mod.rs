use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use components::OnGameScreen;
use network::NetworkPlugin;
use samloc::despawn_screen;
use systems::*;

use crate::states::AppStates;

pub mod components;
mod network;
pub mod resources;
mod systems;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppStates::InGame), game_setup)
            .add_plugins(NetworkPlugin)
            .add_systems(Update, game.run_if(in_state(AppStates::InGame)))
            .add_systems(OnExit(AppStates::InGame), despawn_screen::<OnGameScreen>);
    }
}
