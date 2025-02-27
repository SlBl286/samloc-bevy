use bevy::state::state::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash,Default)]
pub enum AppStates {
    #[default]
    LoadingScreen,
    MainMenu,
    InGame,
}