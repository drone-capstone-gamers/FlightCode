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
    sendEOF: bool,
    phase: Phase,
    index: u8,
    hasPendingByte: bool,
    pendingByte: u8,
    crc: CRC8
}

impl FrameSerializer {
    // Default of sendEOF should be true
    pub fn new(frame: Frame, sendEOF: bool) -> Self {
        Self {
            frame,
            sendEOF,
            phase: Phase::SOF,
            index: 0,
            hasPendingByte: false,
            pendingByte: 0,
            crc: (CRC8::new()),
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::SOF;
        self.index = 0;
        self.hasPendingByte = false;
        self.pendingByte = 0;
        self.crc.reset();
    }

    pub fn hasNext(&self) -> bool {
        self.hasPendingByte || self.phase != Phase::END
    }

    fn handleByteStuffing(&mut self, input: u8) -> u8 {
        if input == LEADING_FLAG {
            self.hasPendingByte = true;
            self.pendingByte = Byte0x55 as u8;
            return ESCAPE_FLAG;
        }
        if input == ESCAPE_FLAG {
            self.hasPendingByte = true;
            self.pendingByte = Byte0xaa as u8;
            return ESCAPE_FLAG;
        }
        return input;
    }
}

impl Iterator for FrameSerializer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.hasNext() {
            return None;
        }

        let mut nextByte: u8 = 0;

        if self.hasPendingByte {
            self.hasPendingByte = false;
            nextByte = self.pendingByte;
        } else {
            match self.phase {
                Phase::SOF => {
                    self.phase = Phase::HEADER;
                    nextByte =  LEADING_FLAG;
                }
                Phase::HEADER => {
                    self.phase = {
                        if self.frame.getPayloadLength() > 0 {
                            Phase::PAYLOAD
                        } else {
                            Phase::CRC
                        }
                    };
                    let b = (self.frame.getService() << HEADER_SERVICE_BIT_SHIFT) | self.frame.getPayloadLength();
                    self.crc.write(b);
                    nextByte = self.handleByteStuffing(b);
                }
                Phase::PAYLOAD => {
                    if self.index >= self.frame.getPayloadLength() - 1 {
                        self.phase = Phase::CRC;
                    }
                    let b = self.frame.getPayload()[self.index as usize];
                    self.index += 1;
                    self.crc.write(b);
                    nextByte = self.handleByteStuffing(b);
                }
                Phase::CRC => {
                    self.phase = {
                        if self.sendEOF {
                            Phase::EOF
                        } else {
                            Phase::END
                        }
                    };
                    let b = self.crc.getCRC();
                    nextByte = self.handleByteStuffing(b);
                }
                Phase::EOF => {
                    self.phase = Phase::END;
                    nextByte = CLOSING_FLAG;
                }
                Phase::END => {
                    panic!("Unexpected read at the end of frame.");
                }
            }
        }

        return Some(nextByte);
    }
}