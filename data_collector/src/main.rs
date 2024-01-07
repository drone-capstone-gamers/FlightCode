mod LinkFraming {
    pub mod Frame {
        const LEADING_FLAG: u8 = 0x55;
        const CLOSING_FLAG: u8 = 0x55;
        const ESCAPE_FLAG: u8 = 0xAA;

        enum EscapeCodes {
            BYTE_0X55 = 0x05,
            BYTE_0XAA = 0x0A
        }

        const HEADER_SERVICE_MASK: u8 = 0xC0;
        const HEADER_PAYLOAD_LENGTH_MASK: u8 = 0x3F;

        pub struct Content {
            service: u8,
            payload: Vec<u8>
        }

        pub fn initialize(frame: &mut Content, service: u8, payload: &mut Vec<u8>) {
            frame.service = service;
            frame.payload.append(payload);
        }
    }

    implement  Deserializer {

    }
}

fn main() {
    let mut frame: LinkFraming::Frame::Content;
    LinkFraming::Frame::initialize(&mut frame, 1, ![0x55, 0x66, 0x77, 0x88, 0x99, 0xAA]);

    println!("Hello, world!");
}
