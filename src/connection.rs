use super::netmsg::*;
use super::world::*;
use log::info;
use serde::Serialize;
use std::io::{Read, Write};
use std::net::TcpStream;
use terview::Message;

/// All the needed info for a player client connection.
pub struct Connection {
    username: String,
    actor_k: ActorKey,
    stream: TcpStream,
}

impl Connection {
    pub fn send<M: Message>(&mut self, msg: NetMsg<M>) {
        self.stream
            .write_all(ron::ser::to_string(&msg).unwrap().as_bytes())
            .unwrap_or_else(|e| {
                panic!("Could not send message to {} client: {}", self.username, e)
            });
    }
}

/// Handle incoming registration request from client.
pub fn register_connection(mut stream: TcpStream, world: &mut World) -> Result<Connection, &'static str> {
    info!("Handling registration attempt...");

    let mut buffer = [0; 512];

    // Read client registration info.
    // TODO: not scalable... avoid using raw byte strings for communication.
    stream.read_exact(&mut buffer).unwrap();
    let username: String = String::from_utf8_lossy(&buffer).into();

    stream.read_exact(&mut buffer).unwrap();
    let role: Role =
        ron::de::from_bytes(&buffer).expect("Received malformed role during player registration");

    info!("Registration successful!: {} (as {:?})", username, role);
    Ok(Connection {
        username,
        actor_k: world.add_actor(username, role)?,
        stream,
    })
}
