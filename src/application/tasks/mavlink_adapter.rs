use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::time::Duration;
use envconfig::Envconfig;
use json::{JsonValue};
use mavlink::common::{HEARTBEAT_DATA, MavAutopilot, MavMessage, MavModeFlag, MavState, MavType};
use mavlink::{MavConnection};
use crate::application::data_manage::{DataSource, get_data_source_string, IncomingData};
use crate::application::timer::TimedTask;

#[derive(Envconfig)]
struct MavlinkConfig {
    #[envconfig(from = "PIXHAWK_PORT", default = "serial:/dev/ttyAMA2:57600")]
    pub pixhawk_port: String
}

pub struct MavlinkAdapter {
    storage_sender: SyncSender<IncomingData>,
    command_recv: Receiver<MavMessage>,
    mavlink_connection: Option<Arc<Box<dyn MavConnection<MavMessage> + Sync + Send>>>,
    heartbeat_lock: bool
}

impl MavlinkAdapter {
    pub fn new(storage_sender: SyncSender<IncomingData>, command_recv: Receiver<MavMessage>) -> Self {
        Self {
            storage_sender,
            command_recv,
            mavlink_connection: None,
            heartbeat_lock: false
        }
    }

    fn launch_heartbeat_thread(&mut self) {
        if self.heartbeat_lock == false {
            self.heartbeat_lock = true;

            thread::spawn({
                let heartbeat = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
                    custom_mode: 0,
                    mavtype: MavType::MAV_TYPE_ONBOARD_CONTROLLER,
                    autopilot: MavAutopilot::MAV_AUTOPILOT_GENERIC,
                    base_mode: MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED,
                    system_status: MavState::MAV_STATE_ACTIVE,
                    mavlink_version: 0x3,
                });

                let connection = self.mavlink_connection.as_mut().unwrap().clone();

                move || loop {
                    let res = connection.send_default(&heartbeat);
                    if res.is_err() {
                        println!("Failed to send initial heartbeat to PixHawk: {res:?}");
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }
    }

    fn send_in_message_to_storage(&mut self, msg: String) {
        let serialized = JsonValue::from(msg);

        let payload = IncomingData::new(DataSource::GlobalPosition, Option::from(serialized), None);
        self.storage_sender.send(payload)
            .expect(&*format!("Failed to send data into write queue: {}",
                              get_data_source_string(&DataSource::GlobalPosition)));
    }
}

impl TimedTask for MavlinkAdapter {
    fn execute(&mut self) -> () {
        if self.mavlink_connection.is_none() {
            let new_connection = mavlink::connect(&MavlinkConfig::init_from_env().unwrap().pixhawk_port);

            if new_connection.is_ok() {
                self.mavlink_connection = Option::from(Arc::new(new_connection.unwrap()));
            } else {
                println!("PixHawk not connected!");
                thread::sleep(Duration::from_secs(10));
                return;
            }

            self.launch_heartbeat_thread();
        }


        let connection = self.mavlink_connection.as_mut().unwrap().clone();
        match connection.recv() {
            Ok((_header, message)) => {
                match message {
                    // Handle received messages as needed
                    MavMessage::COMMAND_LONG(command_long) => {
                        println!("Received COMMAND_LONG: {:?}", command_long);
                    }
                    MavMessage::GLOBAL_POSITION_INT(global_position) => {
                        self.send_in_message_to_storage(serde_json::to_string(&global_position).expect("Failed to serialize MAVLink message to JSON for storage"));
                    }
                    _ => {
                        println!("Received MAVLink message from PixHawk: {:?}", message);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error receiving MAVLink message from PixHawk: {:?}", err);
            }
        }

        let mavlink_command_result = self.command_recv.recv_timeout(Duration::from_millis(100));
        if mavlink_command_result.is_ok() {
            let mavlink_command = mavlink_command_result.unwrap();

            let res = connection.send_default(&mavlink_command);
            if res.is_err() {
                println!("Failed to send command to PixHawk: {res:?}");
            }
        }
    }
}