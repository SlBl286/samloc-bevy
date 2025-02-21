use bevy::prelude::*;

use crate::states::AppStates;

pub fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
  
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/Cards/H2.png")),
        Transform{
            translation: Vec3 { x: 50.0, y: 50.0, z: 0.0 },
            scale: Vec3 { x: 0.5, y: 0.5, z: 0.0 },
            ..Default::default()
        },
    ));
}

// Tick the timer, and change state when finished
pub fn game(
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppStates>>,
) {
}