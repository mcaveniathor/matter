/// Module containing code for protocol messages
pub mod protocol;
use protocol::ProtocolMessage;

bitfield! {
    pub struct MessageFlags(u8);
    impl Debug;
    pub u8, from into DSIZ, dsiz, set_dsiz: 1, 0;
    pub s, set_s: 2;
    pub version, set_version: 7, 4;
}


/// Indicates the size and meaning of Destination Node Id field of [MessageHeader]s.
#[derive(Debug)]
pub enum DSIZ {
    /// Destination Node ID field is not present
    NotPresent,
    /// Destination Node ID is present as a 64.bit Node ID
    NodeID,
    /// Destination Node ID is present as a 16-bit Group ID
    GroupID,
    /// Reserved for future use
    Reserved,
}
impl From<DSIZ> for u8 {
    fn from(dsiz: DSIZ) -> u8 {
        use DSIZ::*;
        match dsiz {
            NotPresent => 0,
            NodeID => 1,
            GroupID => 2,
            _ => 3, // Reserved
        }
    }
}
impl From<u8> for DSIZ {
    fn from(byte: u8) -> DSIZ {
        use DSIZ::*;
        match byte & 0b11 {
            0 => NotPresent,
            1 => NodeID,
            2 => GroupID,
            _ => Reserved,
        }
    }
}

pub type SessionID = u16;

#[derive(Debug)]
pub enum SessionType {
    Unicast,
    Group,
    Reserved,
}
impl From<SessionType> for u8 {
    fn from(session_type: SessionType) -> u8 {
        use SessionType::*;
        match session_type {
            Unicast => 0,
            Group => 1,
            _ => 3, // Reserved
        }
    }
}
impl From<u8> for SessionType {
    fn from(byte: u8) -> SessionType {
        use SessionType::*;
        match byte & 0b11 {
            0 => Unicast,
            1 => Group,
            _ => Reserved,
        }
    }
}

bitfield! {
    pub struct SecurityFlags(u8);
    impl Debug;
    /// Privacy
    pub p, set_p: 7;
    /// Control
    pub c, set_c: 6;
    /// Message Extensions
    pub mx, set_mx: 5;
    /// Session type
    pub u8, from into SessionType, session_type, set_session_type: 1, 0;
}



pub type NodeID = u64;
pub type GroupID = u16;
pub enum DestinationNodeID {
    Node(NodeID),
    Group(GroupID),
}

pub struct MessageHeader {
    /// Overall length of the message in bytes, not including the size of the MessageLength field itself. *SHALL* only be present when the message is being transmitted over a stream-oriented channel such as TCP or QUIC.
    pub message_length: Option<u16>,
    pub message_flags: MessageFlags,
    /// Identifies the session associated with this message
    pub session_id: SessionID,
    pub security_flags: SecurityFlags,
    pub message_counter: u32,
    pub source_node_id: Option<NodeID>,
    pub dest_node_id: Option<DestinationNodeID>,
    pub message_extensions: Option<Vec<u8>>,
}
pub struct MessagePayload {
    pub payload: ProtocolMessage, // NOTE This should be encrypted before transport
}
pub struct MessageFooter {
    pub message_integrity_check: Option<Vec<u8>>,
}
pub struct Message {
    pub header: MessageHeader,
    pub payload: MessagePayload,
    pub footer: MessageFooter,
}


pub enum MessageCounterType {
    GlobalUnencrypted,
    GlobalEncryptedData,
    GlobalEncryptedControl,
    SecureSession,
}
