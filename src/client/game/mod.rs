use bevy::prelude::*;
use components::*;
use lobby::LobbyPlugin;
use samloc::despawn_screen;
use states::InGameState;
use systems::*;

use crate::states::AppStates;

pub mod components;
mod lobby;
pub mod resources;
mod states;
mod systems;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
            app.init_state::<InGameState>()
            .add_systems(OnEnter(AppStates::InGame), game_setup)

            .add_plugins(LobbyPlugin)

            .add_systems(OnExit(AppStates::InGame), despawn_screen::<OnGameScreen>);
    }
}
