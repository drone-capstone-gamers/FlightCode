use crate::crc::CRC8;
use crate::frame::{CLOSING_FLAG, ESCAPE_FLAG, Frame, HEADER_SERVICE_BIT_SHIFT, LEADING_FLAG};
use crate::frame::EscapeCodes::{Byte0x55, Byte0xaa};

#[derive(PartialEq)]
enum Phase {
    SOF,
    HEADER,
    PAYLOAD,
    CRC,
    EOF,
    END
}

pub struct FrameSerializer {
    frame: Frame,
    send_eof: bool,
    phase: Phase,
    index: u8,
    has_pending_byte: bool,
    pending_byte: u8,
    crc: CRC8
}

impl FrameSerializer {
    // Default of sendEOF should be true
    pub fn new(frame: Frame, send_eof: bool) -> Self {
        Self {
            frame,
            send_eof,
            phase: Phase::SOF,
            index: 0,
            has_pending_byte: false,
            pending_byte: 0,
            crc: (CRC8::new()),
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::SOF;
        self.index = 0;
        self.has_pending_byte = false;
        self.pending_byte = 0;
        self.crc.reset();
    }

    pub fn has_next(&self) -> bool {
        self.has_pending_byte || self.phase != Phase::END
    }

    fn handle_byte_stuffing(&mut self, input: u8) -> u8 {
        if input == LEADING_FLAG {
            self.has_pending_byte = true;
            self.pending_byte = Byte0x55 as u8;
            return ESCAPE_FLAG;
        }
        if input == ESCAPE_FLAG {
            self.has_pending_byte = true;
            self.pending_byte = Byte0xaa as u8;
            return ESCAPE_FLAG;
        }
        return input;
    }
}

impl Iterator for FrameSerializer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }

        let next_byte;

        if self.has_pending_byte {
            self.has_pending_byte = false;
            next_byte = self.pending_byte;
        } else {
            match self.phase {
                Phase::SOF => {
                    self.phase = Phase::HEADER;
                    next_byte =  LEADING_FLAG;
                }
                Phase::HEADER => {
                    self.phase = {
                        if self.frame.get_payload_length() > 0 {
                            Phase::PAYLOAD
                        } else {
                            Phase::CRC
                        }
                    };
                    let b = (self.frame.get_service() << HEADER_SERVICE_BIT_SHIFT) | self.frame.get_payload_length();
                    self.crc.write(b);
                    next_byte = self.handle_byte_stuffing(b);
                }
                Phase::PAYLOAD => {
                    if self.index >= self.frame.get_payload_length() - 1 {
                        self.phase = Phase::CRC;
                    }
                    let b = self.frame.get_payload()[self.index as usize];
                    self.index += 1;
                    self.crc.write(b);
                    next_byte = self.handle_byte_stuffing(b);
                }
                Phase::CRC => {
                    self.phase = {
                        if self.send_eof {
                            Phase::EOF
                        } else {
                            Phase::END
                        }
                    };
                    let b = self.crc.get_crc();
                    next_byte = self.handle_byte_stuffing(b);
                }
                Phase::EOF => {
                    self.phase = Phase::END;
                    next_byte = CLOSING_FLAG;
                }
                Phase::END => {
                    panic!("Unexpected read at the end of frame.");
                }
            }
        }

        return Some(next_byte);
    }
}