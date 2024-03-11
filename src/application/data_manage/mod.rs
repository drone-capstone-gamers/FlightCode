mod live_json_stream;

use std::sync::mpsc::Receiver;
use std::{fs, thread};
use std::path::Path;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
use envconfig::Envconfig;
use json::{JsonValue, object};
use crate::application::data_manage::live_json_stream::LiveJsonStream;

#[derive(Envconfig)]
struct DataStorageConfig {
    #[envconfig(from = "FLIGHTCODE_DATA_STORAGE_DIR", default = "./collected_data/")]
    pub target_path: String
}

#[derive(PartialEq, Clone, Copy)]
pub enum DataSource {
    Example = 0,
    GoproImage,
    GlobalPosition,
    IrCamImage,
    Power,
    Temperature,
    Environmental,
    ObcTelemetry,
    PiCamImage,
    Altitude,
    Count,
    Invalid
}

impl DataSource {
    pub const COUNT: usize = DataSource::Count as usize;

    pub fn iter() -> Iter<'static, DataSource> {
        static SOURCES: [DataSource; DataSource::COUNT] = [DataSource::Example,
            DataSource::GoproImage,
            DataSource::GlobalPosition,
            DataSource::IrCamImage,
            DataSource::Power,
            DataSource::Temperature,
            DataSource::Environmental,
            DataSource::ObcTelemetry,
            DataSource::PiCamImage,
            DataSource::Altitude];
        return SOURCES.iter()
    }
}

pub fn get_data_source_string(source: &DataSource) -> String {
    match source {
        DataSource::Example => {"example".to_string()}
        DataSource::GoproImage => {"gopro_image".to_string()}
        DataSource::GlobalPosition => {"global_position".to_string()}
        DataSource::IrCamImage => {"thermal_img".to_string()}
        DataSource::Power => {"power".to_string()}
        DataSource::Temperature => {"temperature".to_string()}
        DataSource::Environmental => {"environmental".to_string()}
        DataSource::ObcTelemetry => {"obc_telemetry".to_string()}
        DataSource::PiCamImage => {"picam_image".to_string()}
        DataSource::Altitude => {"altitude".to_string()}
        _ => {"unsupported".to_string()}
    }
}

pub fn get_data_source_by_name(source_name: String) -> DataSource {
    match source_name.as_str() {
        "example" => {DataSource::Example}
        "gopro_image" => {DataSource::GoproImage}
        "global_position" => {DataSource::GlobalPosition}
        "thermal_img" => {DataSource::IrCamImage}
        "power" => {DataSource::Power}
        "temperature" => {DataSource::Temperature}
        "environmental" => {DataSource::Environmental}
        "obc_telemetry" => {DataSource::ObcTelemetry}
        "picam_image" => {DataSource::PiCamImage}
        "altitude" => {DataSource::Altitude}
        _ => {DataSource::Invalid}
    }
}

// TODO: associate data with mission
#[derive(Clone)]
pub struct IncomingData {
    source: DataSource,
    pub time_stamp: DateTime<Local>,
    pub serialized: Option<JsonValue>,
    pub file: Option<Vec<u8>>
}

impl IncomingData {
    pub fn new(source: DataSource, serialized: Option<JsonValue>, file: Option<Vec<u8>>) -> Self {
        Self {
            source,
            time_stamp: Local::now(),
            serialized,
            file
        }
    }
}

fn create_serialized_file(parent_dir: String, source: DataSource) -> fs::File {
    let source = get_data_source_string(&source);

    return fs::File::create(format!("{}/{}/{}.json", &parent_dir, &source, &source))
        .expect(&*format!("Creation of {}/{}.json failed!", parent_dir, source));
}

fn create_source_directories(storage_dir: String) {
    for source in DataSource::iter() {
        let source_dir = format!("{}/{}", &storage_dir, get_data_source_string(source));
        if !Path::new(&source_dir).exists() {
            fs::create_dir_all(&source_dir).expect(&*format!("Failed to create directory: {}", &storage_dir));
        }
    }
}

struct DataStreams {
    json_streams: [Option<LiveJsonStream>; DataSource::COUNT]
}

impl DataStreams {
    pub fn write_json_stream(&mut self, source_dir: String, source: DataSource, serialized: JsonValue) {
        if self.json_streams[source as usize].is_none() {
            let file = create_serialized_file(source_dir.clone(), source);
            self.json_streams[source as usize] = Option::from(LiveJsonStream::new(file))
        }

        self.json_streams[source as usize].as_mut().unwrap().write(serialized);
    }
}

pub fn spawn_data_manager(data_receiver: Receiver<IncomingData>) -> Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>> {
    const ARRAY_REPEAT_VALUE: Option<IncomingData> = None;
    let current_storage = [ARRAY_REPEAT_VALUE; DataSource::COUNT];
    let current_data_storage = Arc::new(Mutex::new(Box::new(current_storage)));
    let data_use_in_thread = current_data_storage.clone();

    thread::spawn(move || {
        data_manager_loop(data_receiver, data_use_in_thread);
    });

    return current_data_storage;
}

fn data_manager_loop(data_receiver: Receiver<IncomingData>, current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>) {
    let data_storage_config = DataStorageConfig::init_from_env().unwrap();

    let storage_dir = format!("{}/{}", data_storage_config.target_path, Local::now().to_rfc3339());
    create_source_directories(storage_dir.clone());

    const ARRAY_REPEAT_VALUE: Option<LiveJsonStream> = None;
    let mut data_streams = DataStreams { json_streams: [ARRAY_REPEAT_VALUE; DataSource::COUNT] };

    loop {
        let data_result = data_receiver.recv();
        if data_result.is_ok() {
            let incoming_data = data_result.unwrap();

            current_data_storage.lock().unwrap()[incoming_data.source as usize] = Option::from(incoming_data.clone());

            /**
                TODO: Check remaining space on storage drive of destination directory to ensure sufficient storage space
                      and warn on running low
            */
            if incoming_data.serialized.is_some() {
                let serialized = object!{
                    timestamp: incoming_data.time_stamp.to_rfc3339(),
                    data: incoming_data.serialized.unwrap()
                };

                data_streams.write_json_stream(storage_dir.clone(), incoming_data.source, serialized);
            }

            if incoming_data.file.is_some() {
                let file = format!("{}/{}/{}.png", storage_dir.clone(), get_data_source_string(&incoming_data.source), incoming_data.time_stamp.to_rfc3339());
                fs::write(file.clone(), incoming_data.file.unwrap())
                    .expect(&*format!("Failed to write to file: {}", file));
            }
        }
    }
}
