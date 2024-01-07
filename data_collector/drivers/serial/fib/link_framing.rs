mod LinkFraming {
    pub mod Frame {
        const LEADING_FLAG: i8 = 0x55;
        const CLOSING_FLAG: i8 = 0x55;
        const ESCAPE_FLAG: i8 = 0xAA;

        enum EscapeCodes {
            BYTE_0X55 = 0x05,
            BYTE_0XAA = 0x0A
        }

        const HEADER_SERVICE_MASK: i8 = 0xC0;
        const HEADER_PAYLOAD_LENGTH_MASK: i8 = 0x3F;

        pub struct Content {
            service: i8,
            payload: [i8]
        }

        pub fn initialize(frame: &mut Content, service: i8, payloadSize: &i8) {
            frame.service = service;
            frame.payload = [payloadSize]; // <REVIEW>: Double check on if this is how to alloc an array
        }
    }

    enum DeserializerState {
        SEARCH_SOF,
        READ_HEADER,
        READ_PAYLOAD,
        READ_CRC
    }

    trait FrameDeserializer {
        fn next(&self) -> Option<Frame::Content> {
            self.apply(frame, 0x05);
        }

        fn apply(frame: &mut Option<Frame::Content>, byte: i8) {
            return frame;
        }
    }
}

fn main() {
    let frame: LinkFraming::Frame::Content;
    LinkFraming::Frame::Initialize(frame, 1, [0x55, 0x66, 0x77, 0x88, 0x99, 0xAA]);

    println!("Hello, world!");
}
