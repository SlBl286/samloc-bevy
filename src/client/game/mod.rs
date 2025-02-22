use bevy::prelude::*;
use components::OnGameScreen;
use network::NetworkPlugin;
use samloc::despawn_screen;
use states::InGameState;
use systems::*;

use crate::states::AppStates;

pub mod components;
mod lobby;
mod network;
pub mod resources;
mod states;
mod systems;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InGameState>()
            .add_systems(OnEnter(AppStates::InGame), game_setup)
            .add_systems(OnEnter(InGameState::CreateLobby), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_plugins(NetworkPlugin)
            .add_systems(Update, game.run_if(in_state(AppStates::InGame)))
            .add_systems(OnExit(AppStates::InGame), despawn_screen::<OnGameScreen>);
    }
}
