use std::panic::catch_unwind;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::time::Duration;
use envconfig::Envconfig;
use serialport::SerialPort;
use byteorder::{ByteOrder, BigEndian};
use ll_protocol::frame::Frame;
use ll_protocol::frame_deserializer::FrameDeserializer;
use ll_protocol::frame_serializer::FrameSerializer;
use crate::application::data_manage::IncomingData;
use crate::application::DataCollector;
use crate::application::timer::TimedTask;

const POWER_TELEMETRY_SERVICE: u8 = 0;
const TEMPERATURE_TELEMETRY_SERVICE: u8 = 1;
const ENVIRONMENTAL_SENSOR_SERVICE: u8 = 2;
const ACTUATOR_CONTROL_SERVICE: u8 = 3;

const PACKET_IN_POWER_TELEMETRY_LENGTH: u8 = 12;
const PACKET_IN_TEMPERATURE_TELEMETRY_LENGTH: u8 = 20;
const PACKET_IN_ENVIRONMENTAL_SENSOR_LENGTH: u8 = 8;

#[derive(Envconfig)]
struct PibAdapterConfig {
    #[envconfig(from = "PIB_SERIAL_PORT", default = "/dev/ttyACM0")]
    pub serial_port: String
}

pub struct PibAdapter {
    receiving_active: bool,
    serial: Option<Box<dyn SerialPort>>,
    frame_deserializer: FrameDeserializer,
    serial_buf: Vec<u8>,
    storage_sender: SyncSender<IncomingData>
}

impl PibAdapter {
    fn handle_in_frame(frame: Frame) {
        match frame.get_service() {
            POWER_TELEMETRY_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_POWER_TELEMETRY_LENGTH {
                    // TODO: Log an error
                    return;
                }
            },
            TEMPERATURE_TELEMETRY_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_TEMPERATURE_TELEMETRY_LENGTH {
                    // TODO: Log an error
                    return;
                }
            },
            ENVIRONMENTAL_SENSOR_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_ENVIRONMENTAL_SENSOR_LENGTH {
                    // TODO: Log an error
                    return;
                }
            },
            _ => {
                // TODO: log invalid in-service num
            }
        }
    }

    fn parse_float(bytes: [u8; 4]) -> f32 {
        return BigEndian::read_f32(&bytes);
    }
}

impl DataCollector for PibAdapter {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self {
        Self {
            receiving_active: false,
            frame_deserializer: FrameDeserializer::new(),
            serial_buf: vec![0; 128],
            serial: None,
            storage_sender
        }
    }
}

impl TimedTask for PibAdapter {
    fn execute(&mut self) -> () {
        if self.serial.is_none() {
            let new_port = serialport::new(&PibAdapterConfig::init_from_env().unwrap().serial_port, 115_200)
                                            .timeout(Duration::from_millis(3))
                                            .open();

            if new_port.is_ok() {
                self.serial = Option::from(new_port.unwrap());
            } else {
                println!("PIP port not connected!");
                return;
            }
        }

        let serial = self.serial.as_mut().unwrap();

        let bytes_to_read = serial.bytes_to_read().unwrap();

        if bytes_to_read > 0 {
            serial.read(self.serial_buf.as_mut_slice()).expect("Found no data!");

            self.serial_buf.iter().map(|&byte| self.frame_deserializer.apply(byte))
                .filter(|result| result.is_some())
                .for_each(|result| {
                    let deserialized_frame = result.unwrap();
                    PibAdapter::handle_in_frame(deserialized_frame);
                });
        }
    }
}


