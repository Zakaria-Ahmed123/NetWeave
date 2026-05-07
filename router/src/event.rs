#[allow(dead_code)]
#[derive(Debug)]
pub enum LanEvent {
    PacketFromPeer(Vec<u8>),
    NewPeerOffer(String, String),
    ChatMessage { from: String, message: String },
    PeerConnected(String),
    PeerDisconnected(String),
}
