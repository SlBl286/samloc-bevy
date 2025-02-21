use std::{net::*, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{netcode::*, renet::*, *};


pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);
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
    }
}