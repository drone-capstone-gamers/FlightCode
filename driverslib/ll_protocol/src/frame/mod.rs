use std::fmt::Display;

pub const LEADING_FLAG: u8 = 0x55;
pub const CLOSING_FLAG: u8 = 0x55;
pub const ESCAPE_FLAG: u8 = 0xAA;

pub enum EscapeCodes {
    Byte0x55 = 0x05,
    Byte0xaa = 0x0A
}

pub const HEADER_PAYLOAD_LENGTH_MASK: u8 = 0x3F;

pub const HEADER_SERVICE_BIT_SHIFT: i32 = 6;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn get_payload_length(&self) -> u8 {
        self.payload.len() as u8
    }
    pub fn get_service(&self) -> u8 {
        self.service
    }
    pub fn get_payload(&mut self) -> &mut Vec<u8> {
        &mut self.payload
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = String::from("Frame[service=");
        builder += &*self.service.to_string();

        builder += &*", payload=";
        if self.get_payload_length() == 0 {
            builder += &*"(ZLP)";
        } else {
            for byte in &self.payload {
                builder += &*format!("{:#03x},", byte);
            }
            builder.pop();
        }
        builder += &*"]";

        return write!(f, "{}", builder);
    }
}
