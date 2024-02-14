use std::sync::mpsc::SyncSender;
use std::time::Duration;
use envconfig::Envconfig;
use serialport::SerialPort;
use byteorder::{ByteOrder, BigEndian};
use json::object;
use ll_protocol::frame::Frame;
use ll_protocol::frame_deserializer::FrameDeserializer;
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
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

/**
 * In PIP commands
 */
impl PibAdapter {
    fn handle_in_frame(mut frame: Frame, storage_sender: SyncSender<IncomingData>) {
        match frame.get_service() {
            POWER_TELEMETRY_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_POWER_TELEMETRY_LENGTH {
                    // TODO: Log an error
                    return;
                }

                let av_volt = BigEndian::read_f32(&frame.get_payload()[0..3]);
                let av_cur = BigEndian::read_f32(&frame.get_payload()[4..7]);
                let av_pow = BigEndian::read_f32(&frame.get_payload()[8..11]);

                let payload = object!{
                    average_voltage: av_volt,
                    average_current: av_cur,
                    average_power: av_pow
                };

                let data_payload = IncomingData::new(DataSource::Power, Option::from(payload), None);
                storage_sender.send(data_payload)
                    .expect(&*format!("Failed to send data into write queue: {}",
                                      get_data_source_string(DataSource::Power)));
            },
            TEMPERATURE_TELEMETRY_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_TEMPERATURE_TELEMETRY_LENGTH {
                    // TODO: Log an error
                    return;
                }

                let pow_converter_temp = BigEndian::read_f32(&frame.get_payload()[0..3]);
                let esc_1_temp = BigEndian::read_f32(&frame.get_payload()[4..7]);
                let esc_2_temp = BigEndian::read_f32(&frame.get_payload()[8..11]);
                let esc_3_temp = BigEndian::read_f32(&frame.get_payload()[12..15]);
                let esc_4_temp = BigEndian::read_f32(&frame.get_payload()[16..19]);

                let payload = object!{
                    power_converter_temperature: pow_converter_temp,
                    esc_1_temperature: esc_1_temp,
                    esc_2_temperature: esc_2_temp,
                    esc_3_temperature: esc_3_temp,
                    esc_4_temperature: esc_4_temp
                };

                let data_payload = IncomingData::new(DataSource::Temperature, Option::from(payload), None);
                storage_sender.send(data_payload)
                    .expect(&*format!("Failed to send data into write queue: {}",
                                      get_data_source_string(DataSource::Temperature)));
            },
            ENVIRONMENTAL_SENSOR_SERVICE => {
                if frame.get_payload_length() != PACKET_IN_ENVIRONMENTAL_SENSOR_LENGTH {
                    // TODO: Log an error
                    return;
                }

                let temp = BigEndian::read_f32(&frame.get_payload()[0..3]);
                let hum = BigEndian::read_f32(&frame.get_payload()[4..7]);

                let payload = object!{
                    temperature: temp,
                    humidity: hum
                };

                let data_payload = IncomingData::new(DataSource::Environmental, Option::from(payload), None);
                storage_sender.send(data_payload)
                    .expect(&*format!("Failed to send data into write queue: {}",
                                      get_data_source_string(DataSource::Environmental)));
            },
            _ => {
                // TODO: log invalid in-service num
            }
        }
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
            let new_port = serialport::new(&PibAdapterConfig::init_from_env().unwrap().serial_port, 9_600)
                                            .timeout(Duration::from_millis(3))
                                            .open();

            if new_port.is_ok() {
                self.serial = Option::from(new_port.unwrap());
            } else {
                println!("PIB port not connected!");
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
                    PibAdapter::handle_in_frame(deserialized_frame, self.storage_sender.clone());
                });
        }
    }
}

/**
* Out PIP commands
*/
impl PibAdapter {
    fn handle_out_frame() {

    }

    pub fn put_power_set_rate(rate: u8) {

    }

    pub fn get_power_request() {

    }

    pub fn put_temperature_set_rate(rate: u8) {

    }

    pub fn get_temperature__request() {

    }

    pub fn put_environmental_set_rate(rate: u8) {

    }

    pub fn get_environmental__request() {

    }

    pub fn put_servo_stop() {

    }

    pub fn put_servo_set(pwm: u8) {

    }

    pub fn put_indicator_light_set() {

    }
}
