mod states;
mod contants;

use std::{ net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_renet::{netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport}, renet::{ ConnectionConfig, DefaultChannel, RenetClient}, RenetClientPlugin};
use contants::*;
use rand::random;

use samloc::{ClientChannel, PlayerCommand, ServerMessages};

use states::AppStates;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.init_state::<AppStates>();
    let client = RenetClient::new(ConnectionConfig::default());
    app.insert_resource(client);
    
    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
        client_id,
        user_data: None,
        protocol_id: 0,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(transport);
    app.add_systems(OnEnter(AppStates::LoadingScreen), initial_setup);
    app.add_systems(Update, transition_to_in_menu.run_if(in_state(AppStates::LoadingScreen)));
    app.add_systems(OnEnter(AppStates::MainMenu), relies_on_initial_setup);
    app.add_systems(Update, button_system);
    app.add_systems(Update,receive_message_system);
    app.run();
}

// Systems
fn initial_setup(){

}
fn relies_on_initial_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(Camera2d);
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height:Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        column_gap: Val::Px(5.0),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new("Button"),
            TextFont {
                font: asset_server.load("fonts/PatrickHand-Regular.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9,0.9,0.9)),
        ));
        parent.spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new("Button"),
            TextFont {
                font: asset_server.load("fonts/PatrickHand-Regular.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9,0.9,0.9)),
        ));
    });
}
fn transition_to_in_menu(mut app_state: ResMut<NextState<AppStates>>) {
    app_state.set(AppStates::MainMenu);
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message: ServerMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::Message { value } => {
                println!("{}",value);
            }
            _ => {}
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut client: ResMut<RenetClient>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                let message = bincode::serialize(&PlayerCommand::JoinLobby { lobby_id: String::from("ABC123")}).unwrap();
                client.send_message( ClientChannel::Command,message );
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}