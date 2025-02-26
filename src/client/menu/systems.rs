use bevy::prelude::*;

use crate::{
    contants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, TEXT_COLOR},
    menu::enums::MenuButton,
    states::AppStates,
};

use super::{components::*, states::*};
pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("SamLoc"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButton::Create,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/UI/create-lobby.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("Create"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButton::Join,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/UI/join-lobby.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("Join"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButton::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/UI/settings.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButton::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("sprites/UI/exit.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node));
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font,
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}
pub fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppStates>>,
    music_box_query: Query<&AudioSink, With<ClickedAudio>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(sink) = music_box_query.get_single() {
                sink.play();
            }
            match menu_button_action {
                MenuButton::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButton::Create => {
                    app_state.set(AppStates::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButton::Join => {
                    app_state.set(AppStates::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButton::Settings => menu_state.set(MenuState::Settings),
                // MenuButton::SettingsDisplay => {
                //     menu_state.set(MenuState::SettingsDisplay);
                // }
                // MenuButton::SettingsSound => {
                //     menu_state.set(MenuState::SettingsSound);
                // }
                // MenuButton::BackToMainMenu => menu_state.set(MenuState::Main),
                // MenuButton::BackToSettings => {
                //     menu_state.set(MenuState::Settings);
                // }
            }
        }
    }
}
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
pub fn spawn_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    let audio = asset_server.load("sounds/click-a.ogg");

    // Create an entity dedicated to playing our background music
    commands.spawn((
        ClickedAudio,
        AudioPlayer::new(audio),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Once,
            paused: true,
            ..default()
        },
    ));
}
