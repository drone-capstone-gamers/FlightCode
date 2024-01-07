use crate::crc::CRC8;
use crate::frame::{ESCAPE_FLAG, Frame, HEADER_PAYLOAD_LENGTH_MASK, HEADER_SERVICE_BIT_SHIFT, LEADING_FLAG};

#[derive(PartialEq, Eq)]
enum State {
    SearchSof,
    ReadHeader,
    ReadPayload,
    ReadCrc,
}

pub struct FrameDeserializer {
    state: State,
    frame: Option<Frame>,
    index: usize,
    crc: CRC8,
    stuff_byte: bool,
}

impl FrameDeserializer {
    pub fn new() -> Self {
        FrameDeserializer {
            state: State::SearchSof,
            frame: None,
            index: 0,
            crc: CRC8::new(),
            stuff_byte: false,
        }
    }

    pub fn reset(&mut self) {
        self.state = State::SearchSof;
        self.frame = None;
        self.index = 0;
        self.crc.reset();
        self.stuff_byte = false;
    }

    fn apply_core(&mut self, input: u8) -> Option<Frame> {
        match self.state {
            State::SearchSof => None,
            State::ReadHeader => {
                let len = input & HEADER_PAYLOAD_LENGTH_MASK;
                let service = input >> HEADER_SERVICE_BIT_SHIFT;
                self.frame = Some(Frame::new(service, vec![0; len as usize]));
                self.index = 0;
                self.crc.write(input);
                self.state = if len > 0 { State::ReadPayload } else { State::ReadCrc };
                None
            }
            State::ReadPayload => {
                self.crc.write(input);
                let frame = self.frame.as_mut().unwrap();
                let payload: &mut Vec<u8> = frame.get_payload();
                payload[self.index] = input;
                self.index += 1;
                if self.index >= payload.len() {
                    self.state = State::ReadCrc;
                }
                None
            }
            State::ReadCrc => {
                self.crc.write(input);
                if self.crc.get_crc() != 0x00 {
                    // CRC verification failed
                    self.reset();
                    None
                } else {
                    // CRC verification passed
                    let frame = self.frame.take();
                    self.reset();
                    frame
                }
            }
        }
    }

    pub fn apply(&mut self, input: u8) -> Option<Frame> {
        if input == LEADING_FLAG {
            // Framing byte, attempt to read headers
            self.reset();
            self.state = State::ReadHeader;
            None
        } else if self.stuff_byte {
            // Read escaped byte
            self.stuff_byte = false;
            match input {
                0x05 => self.apply_core(0x55),
                0x0A => self.apply_core(0xAA),
                _ => {
                    // Invalid escaped byte
                    self.reset();
                    None
                }
            }
        } else if input == ESCAPE_FLAG {
            // The next byte is the escaped byte
            self.stuff_byte = true;
            None
        } else {
            self.apply_core(input)
        }
    }
}