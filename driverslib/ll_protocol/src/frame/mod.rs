pub const LEADING_FLAG: u8 = 0x55;
pub const CLOSING_FLAG: u8 = 0x55;
pub const ESCAPE_FLAG: u8 = 0xAA;

pub enum EscapeCodes {
    Byte0x55 = 0x05,
    Byte0xaa = 0x0A
}

pub const HEADER_SERVICE_MASK: u8 = 0xC0;
pub const HEADER_PAYLOAD_LENGTH_MASK: u8 = 0x3F;

pub const HEADER_SERVICE_BIT_SHIFT: i32 = 6;

pub struct Frame {
    service: u8,
    payload: Vec<u8>
}

impl Frame {
    pub fn new(service: u8, payload: Vec<u8>) -> Self {
        Self {
            service,
            payload: (payload.clone())
        }
    }

    pub fn getPayloadLength(&self) -> u8 {
        self.payload.len() as u8
    }
    pub fn getService(&self) -> u8 {
        self.service
    }
    pub fn getPayload(&self) -> &Vec<u8> {
        &self.payload
    }
}