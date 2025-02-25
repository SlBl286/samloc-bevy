use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig}, renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent}, RenetServerPlugin};
use samloc::{ClientChannel, PlayerCommand, ServerMessages};



fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    let server_addr = "0.0.0.0:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);
    
    app.add_systems(Update,handle_events_system);
    app.add_systems(Update,send_message_system);
    app.add_systems(Update,receive_message_system);
    app.run();
}

// Systems

fn send_message_system() {

}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let client_message: PlayerCommand = bincode::deserialize(&message).unwrap();
            match client_message {
                PlayerCommand::JoinLobby { lobby_id } => {
                    println!("player {} join lobby {}",client_id,lobby_id);
                }
                _ => {}
            }
        }
    }
}

fn handle_events_system(mut server: ResMut<RenetServer>,mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let message = bincode::serialize(&ServerMessages::Message { value: String::from(format!("wellcome {} to server",client_id)) } ).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}