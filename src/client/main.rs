mod states;
mod contants;

mod loading;
mod menu;
mod game;

use std::{ net::*, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{netcode::*, renet::*, RenetClientPlugin};

use game::InGamePlugin;
use loading::LoadingScreenPlugin;
use menu::MenuScreenPlugin;
use samloc::ServerMessages;

use states::AppStates;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
   

    app.init_state::<AppStates>();
    app.add_systems(Startup, startup);
    app.add_plugins(LoadingScreenPlugin);
    app.add_plugins(MenuScreenPlugin);
    app.add_plugins(InGamePlugin);
    app.add_systems(Update,receive_message_system);
    app.run();
}

// Systems
fn startup ( mut commands: Commands,){
    commands.spawn(Camera2d);
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

