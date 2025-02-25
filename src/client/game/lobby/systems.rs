use bevy::{prelude::*, text::cosmic_text::ttf_parser::Style};

use crate::contants::TEXT_COLOR;

use super::{components::*, events::SliderMoveEvent};

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
            parent
                .spawn((Node { ..default() },))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode {
                            image: asset_server
                                .load("sprites/UI/PNG/Default/slide_horizontal_grey.png"),
                            ..default()
                        },
                        Node {
                            height: Val::Px(16.0),
                            width: Val::Px(96.0),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        ImageNode {
                            image: asset_server
                                .load("sprites/UI/PNG/Default/slide_horizontal_grey.png"),
                            ..default()
                        },
                        Node {
                            height: Val::Px(16.0),
                            width: Val::Px(96.0),
                            left: Val::Px(-16.0),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        ImageNode {
                            image: asset_server
                                .load("sprites/UI/PNG/Default/slide_horizontal_grey.png"),
                            ..default()
                        },
                        Node {
                            height: Val::Px(16.0),
                            width: Val::Px(96.0),
                            left: Val::Px(-32.0),
                            ..default()
                        },
                    ));
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageNode {
                                image: asset_server.load("sprites/UI/PNG/Default/slide_hangle.png"),
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
                        ))
                        // .observe(down_handler)
                        // .observe(up_handler)
                        // .observe(drag_start)
                        .observe(drag_handler);
                        // .observe(drag_end);
                });

            parent
                .spawn((
                    Button,
                    ImageNode::new(
                        asset_server.load("sprites/UI/PNG/Default/button_rectangle_depth_flat.png"),
                    ),
                    Node { ..default() },
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

// pub fn down_handler(_: Trigger<Pointer<Down>>) {
//     info!("Down");
// }

// pub fn up_handler(_: Trigger<Pointer<Up>>) {
//     info!("Up");
// }

// pub fn drag_start(_: Trigger<Pointer<DragStart>>) {
//     info!("Start");
// }

pub fn drag_handler(drag_info: Trigger<Pointer<Drag>>,mut ev_slider_move: EventWriter<SliderMoveEvent>) {
    info!("{}",drag_info.delta);
    ev_slider_move.send(SliderMoveEvent(drag_info.distance.x));
}

// pub fn drag_end(_: Trigger<Pointer<DragEnd>>) {
//     info!("End");
// }

pub fn handle_slider_move(
    mut ev_slider_move: EventReader<SliderMoveEvent>,
    mut node: Query<&mut Node, With<SliderHangle>>,
) {
    let mut node = node.single_mut();
    for ev in ev_slider_move.read() {
        if let Val::Px(left) = &mut node.left {
            if ev.0 > 0.0 && *left < 0.0 {
                *left += 64.0;
            }
            else if ev.0 < -0.0 && *left > -96.0 * 3.0 {
                *left -= 64.0;
            }
        }
        
    }
}