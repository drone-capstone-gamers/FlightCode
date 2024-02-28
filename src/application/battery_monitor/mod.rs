use std::sync::{Arc, Mutex};
use std::sync::mpsc::SyncSender;
use mavlink::common::MavMessage;
use crate::application::data_manage::{DataSource, IncomingData};

pub fn spawn_battery_monitor(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, mavlink_commander: SyncSender<MavMessage>) {

}