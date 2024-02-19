mod live_json_stream;

use std::sync::mpsc::Receiver;
use std::{fs, thread};
use std::path::Path;
use std::slice::Iter;
use chrono::{DateTime, Local};
use envconfig::Envconfig;
use json::{JsonValue, object};
use crate::application::data_manage::live_json_stream::LiveJsonStream;

#[derive(Envconfig)]
struct DataStorageConfig {
    #[envconfig(from = "FLIGHTCODE_DATA_STORAGE_DIR", default = "./collected_data/")]
    pub target_path: String
}

#[derive(PartialEq)]
pub enum DataSource {
    Example,
    GoproImage,
    Gps,
    IrCamImage,
    Power,
    Temperature,
    Environmental
}

impl DataSource {
    pub fn iter() -> Iter<'static, DataSource> {
        static SOURCES: [DataSource; 7] = [DataSource::Example,
            DataSource::GoproImage,
            DataSource::Gps,
            DataSource::IrCamImage,
            DataSource::Power,
            DataSource::Power,
            DataSource::Environmental];
        return SOURCES.iter()
    }
}


pub fn get_data_source_string(source: &DataSource) -> String {
    match source {
        DataSource::Example => {"example".to_string()}
        DataSource::GoproImage => {"high_res_img".to_string()}
        DataSource::Gps => {"gps".to_string()}
        DataSource::IrCamImage => {"thermal_img".to_string()}
        DataSource::Power => {"power".to_string()}
        DataSource::Temperature => {"temperature".to_string()}
        DataSource::Environmental => {"environmental".to_string()}
    }
}

// TODO: associate data with mission
pub struct IncomingData {
    source: DataSource,
    time_stamp: DateTime<Local>,
    serialized: Option<JsonValue>,
    file: Option<Vec<u8>>
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

    return fs::File::create(format!("{}/{}.json", &parent_dir, &source))
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

pub fn spawn_data_manager(data_receiver: Receiver<IncomingData>) {
    thread::spawn(|| {
        data_manager_loop(data_receiver);
    });
}

fn data_manager_loop(data_receiver: Receiver<IncomingData>) {
    let data_storage_config = DataStorageConfig::init_from_env().unwrap();

    let storage_dir = format!("{}/{}", data_storage_config.target_path, Local::now().to_rfc3339());
    create_source_directories(storage_dir.clone());

    // TODO: better setup serialized file streams
    let example_file = create_serialized_file(storage_dir.clone(), DataSource::Example);
    let mut example_json_stream = LiveJsonStream::new(example_file);

    loop {
        let data_result = data_receiver.recv();
        if data_result.is_ok() {
            let incoming_data = data_result.unwrap();

            /**
                TODO: Check remaining space on storage drive of destination directory to ensure sufficient storage space
                      and warn on running low
            */
            if incoming_data.serialized.is_some() {
                let serialized = object!{
                    timestamp: incoming_data.time_stamp.to_rfc3339(),
                    data: incoming_data.serialized.unwrap()
                };

                example_json_stream.write(serialized);
            }

            if incoming_data.file.is_some() {
                let file = format!("{}/{}/{}.png", storage_dir.clone(), get_data_source_string(&incoming_data.source), incoming_data.time_stamp.to_rfc3339());
                fs::write(file.clone(), incoming_data.file.unwrap())
                    .expect(&*format!("Failed to write to file: {}", file));
            }
        }
    }
}
