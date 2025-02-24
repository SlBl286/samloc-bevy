use bevy::prelude::*;
use bevy_renet::renet::*;
use samloc::ServerMessages;


pub fn receive_message_system(mut client: ResMut<RenetClient>) {
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
