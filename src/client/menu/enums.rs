use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButton {
    Create,
    Join,
    Settings,
    Quit,
}