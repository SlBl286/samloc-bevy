use bevy::prelude::*;
use components::*;
use samloc::despawn_screen;
use systems::*;

use super::states::InGameState;

mod systems;
pub mod  components;

pub struct  LobbyPlugin;

impl  Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(InGameState::CreateLobby), create_lobby_setup)
        .add_systems(OnExit(InGameState::CreateLobby), despawn_screen::<OnCreateLobbyScreen>);
    }
}