use bevy::prelude::*;

use crate::contants::TEXT_COLOR;

use super::{components::*, enums::CreateLobbyButton, events::SliderMoveEvent, resources::SliderDelayMoveTimer};

pub fn create_lobby_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
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
                                top: Val::Px(-3.0),
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
                    Node { 
                        width : Val::Auto,
                        height: Val::Px(100.0),
                        ..default()
                     },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SamLoc"),
                        TextFont {
                            font_size: 40.0,
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

pub fn drag_handler(
    drag_info: Trigger<Pointer<Drag>>,
    mut ev_slider_move: EventWriter<SliderMoveEvent>,
) {
    info!("{}", drag_info.delta);
    ev_slider_move.send(SliderMoveEvent(drag_info.distance.x));
}

// pub fn drag_end(_: Trigger<Pointer<DragEnd>>) {
//     info!("End");
// }

pub fn handle_slider_move(
    mut ev_slider_move: EventReader<SliderMoveEvent>,
    mut node: Query<&mut Node, With<SliderHangle>>,
    mut slider_delay: ResMut<SliderDelayMoveTimer>,
    time: Res<Time>,
) {
    let mut node = node.single_mut();
    slider_delay.tick(time.delta());
    if slider_delay.finished() {
        for ev in ev_slider_move.read() {
            if let Val::Px(left) = &mut node.left {
                if ev.0 > 0.0 && *left < -96.0 {
                    *left += 80.0;
                } else if ev.0 < -0.0 && *left > -96.0 * 3.0 {
                    *left -= 80.0;
                }
            }
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (Changed<Interaction>, With<Button>, With<CreateLobbyButton>),
    >,
    asset_server: Res<AssetServer>
) {
    for (interaction, mut image) in &mut interaction_query {
        *image = match *interaction {
            Interaction::Pressed=>  ImageNode::new(
                asset_server.load("sprites/UI/PNG/Default/button_rectangle_flat.png"),
            ),
            Interaction::Hovered=> ImageNode::new(
                asset_server.load("sprites/UI/PNG/Default/button_rectangle_depth_flat.png"),
            ),
            Interaction::None => ImageNode::new(
                asset_server.load("sprites/UI/PNG/Default/button_rectangle_depth_flat.png"),
            ),
        }
    }
}