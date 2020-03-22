use super::world::Role;
use serde::{Deserialize, de::DeserializeOwned, Serialize};

/// Possible messages sent along client-server connection.
#[derive(Deserialize, Serialize)]
pub enum NetMsg<M>
where
    M: terview::Message,
{
    // Test message to be displayed.
    Msg(M),

    // Registration info.
    RegInfo(RegInfo),
}

/// Info needed by the server for registration of a client.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegInfo {
    username: String,
    role: Role,
}
