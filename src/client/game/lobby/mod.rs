use bevy::prelude::*;
use components::*;
use events::SliderMoveEvent;
use samloc::despawn_screen;
use systems::*;

use super::states::InGameState;

mod systems;
pub mod components;
pub mod events;

pub struct  LobbyPlugin;

impl  Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SliderMoveEvent>()
        .add_systems(OnEnter(InGameState::CreateLobby), create_lobby_setup)
        .add_systems(Update, handle_slider_move.run_if(in_state(InGameState::CreateLobby)))
        .add_systems(OnExit(InGameState::CreateLobby), despawn_screen::<OnCreateLobbyScreen>);
    }
}