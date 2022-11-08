/// A Prototol Message as defined in the Matter Specification
pub struct ProtocolMessage {
    pub exchange_flags: ExchangeFlags,
    pub protocol_opcode: u8,
    pub exchange_id: u16,
    pub protocol_id: u16,
    pub protocol_vendor_id: Option<u16>,
    pub acknowledged_message_counter: Option<u32>,
    pub secured_extensions: Option<Vec<u8>>,
    pub payload: Option<Vec<u8>>,
}

bitfield!{
    pub struct ExchangeFlags(u8);
    impl Debug;
    /// Initiator
    pub i, set_i: 0;
    /// Acknowledegment
    pub a, set_a: 1;
    /// Reliability
    pub r, set_r: 2;
    /// Secured Extensions
    pub sx, set_sx: 3;
    /// Vendor
    pub v, set_v: 4;
}
