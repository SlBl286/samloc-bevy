use bevy::state::state::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    CreateLobby,
    JoinLobby,
    InGame,
    #[default]
    Disabled,
}
