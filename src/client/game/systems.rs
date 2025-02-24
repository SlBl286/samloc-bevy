use bevy::prelude::*;

use super::states::InGameState;

pub fn game_setup(mut game_state: ResMut<NextState<InGameState>>) {
    game_state.set(InGameState::CreateLobby);
}
