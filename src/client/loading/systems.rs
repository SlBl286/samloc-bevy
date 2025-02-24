use bevy::prelude::*;

use crate::states::AppStates;

use super::{components::OnLoadingScreen, resources::SplashTimer};

pub fn loading_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("sprites/logo.png");
    // Display the logo
    commands
        .spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            OnLoadingScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(icon),
                Node {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Percent(30.0),
                    ..default()
                },
            ));
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.5, TimerMode::Once)));
}
pub fn countdown(
    mut game_state: ResMut<NextState<AppStates>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppStates::MainMenu);
    }
}