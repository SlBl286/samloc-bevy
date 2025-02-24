mod states;
mod contants;

mod loading;
mod menu;
mod game;
mod network;


use bevy::prelude::*;


use game::InGamePlugin;
use loading::LoadingScreenPlugin;
use menu::MenuScreenPlugin;
use network::NetworkPlugin;

use states::AppStates;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
    .set(WindowPlugin {
        primary_window: Some(Window{
            title: "Sam Loc App".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(NetworkPlugin)
    .init_state::<AppStates>()
    .add_systems(Startup, startup)
    .add_plugins(LoadingScreenPlugin)
    .add_plugins(MenuScreenPlugin)
    .add_plugins(InGamePlugin)
    .run();
}

// Systems
fn startup ( mut commands: Commands,){
    commands.spawn(Camera2d);
}


