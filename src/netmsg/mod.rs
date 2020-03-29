use super::world::Role;
use serde::{Deserialize, Serialize};
use super::world::World;

/// 
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum NetErr {
    InexistentPlayerName,
}

/// Possible messages sent along client-server connection.
#[derive(Deserialize, Serialize)]
pub enum NetMsg<M>
where
    M: terview::Message,
{
    PosAwk,
    Err(NetErr),
    Msg(M),
    World(World),
}


/// Info needed by the server for registration of a client.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegInfo {
    username: String,
    role: Role,
}
