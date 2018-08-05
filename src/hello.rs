/// Represents a ClientHello message (ID 0x01)
#[derive(Debug, ClientMessage)]
#[packet_id = 0x01]
pub struct ClientHello {
    rev: u16,
    client_name: String
}

impl ClientHello {
    pub fn new(rev: u16, client_name: &str) -> ClientHello {
        ClientHello {
            rev,
            client_name: client_name.to_string()
        }
    }
}
