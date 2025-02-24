use bevy::prelude::*;

use crate::contants::TEXT_COLOR;

use super::components::*;

pub fn create_lobby_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnCreateLobbyScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node{

                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                .spawn((
                    ImageNode {
                        image: asset_server
                            .load("sprites/UI/PNG/Blue/Default/slide_horizontal_grey.png"),
                        ..default()
                    },
                    Node {
                        height: Val::Px(16.0),
                        width: Val::Px(96.0),
                        ..default()
                    },
                ));
                parent
                .spawn((
                    ImageNode {
                        image: asset_server
                            .load("sprites/UI/PNG/Blue/Default/slide_horizontal_grey.png"),
                        ..default()
                    },
                    Node {
                        height: Val::Px(16.0),
                        width: Val::Px(96.0),
                        left: Val::Px(-16.0),
                        ..default()
                    },
                ));
                parent
                .spawn((
                    ImageNode {
                        image: asset_server
                            .load("sprites/UI/PNG/Blue/Default/slide_horizontal_grey.png"),
                        ..default()
                    },
                    Node {
                        height: Val::Px(16.0),
                        width: Val::Px(96.0),
                        left: Val::Px(-32.0),
                        ..default()
                    },
                ));
            }).with_child((
                ImageNode {
                    image: asset_server
                        .load("sprites/UI/PNG/Blue/Default/slide_hangle.png"),
                    ..default()
                },
                Node {
                    height: Val::Px(20.0),
                    width: Val::Px(15.0),
                    left: Val::Px(-96.0 * 3.0),
                    top: Val::Px(-2.0),
                    ..default()
                },
                SliderHangle,
            ));

            parent.spawn((
                Button,
                ImageNode::new(asset_server.load("sprites/UI/PNG/Blue/Default/button_rectangle_depth_flat.png")),
                Node {
                    ..default()
                },
            ))
            .with_children(|parent| {
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
            });
        });
}

pub fn down_handler(_: Trigger<Pointer<Down>>) {
    info!("Down");
}



