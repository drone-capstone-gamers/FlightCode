use std::sync::mpsc::Receiver;
use std::{fs, thread};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use chrono::{DateTime, Local};
use envconfig::Envconfig;
use json::{JsonValue, object};

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
    TempHumidity
}

pub fn get_data_source_string(source: DataSource) -> String {
    match source {
        DataSource::Example => {"example".to_string()}
        DataSource::GoproImage => {"high_res_img".to_string()}
        DataSource::Gps => {"gps".to_string()}
        DataSource::IrCamImage => {"thermal_img".to_string()}
        DataSource::TempHumidity => {"temp_humidity".to_string()}
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
    let source = get_data_source_string(source);

    return fs::File::create(format!("{}/{}.json", &parent_dir, &source))
        .expect(&*format!("Creation of {}/{}.json failed!", parent_dir, source));
}

pub fn spawn_data_manager(data_receiver: Receiver<IncomingData>) {
    thread::spawn(|| {
        data_manager_loop(data_receiver);
    });
}

fn data_manager_loop(data_receiver: Receiver<IncomingData>) {
    let mut geotag: Option<JsonValue> = None;

    let data_storage_config = DataStorageConfig::init_from_env().unwrap();

    let storage_dir = format!("{}/{}", data_storage_config.target_path, Local::now().to_rfc3339());
    if !Path::new(&storage_dir).exists() {
        fs::create_dir_all(&storage_dir).expect(&*format!("Failed to create directory: {}", &storage_dir));
    }

    // TODO: ABSTRACT LIVE JSON WRITING TO JSON AS IO STREAM TO MODULE AND FIX
    let mut serialized_file = create_serialized_file(storage_dir.clone(), DataSource::Example);

    // Setup json file with { at start, && are dummy characters to be removed further down
    serialized_file.write("{\"data_stream\": [\n&&".as_bytes()).expect("Failed to initialize data capture fail for unknown reason");

    loop {
        let data_result = data_receiver.recv();
        if data_result.is_ok() {
            let incoming_data = data_result.unwrap();

            // Record GPS coords into local variable to use for geotagging images
            if incoming_data.source == DataSource::Gps {
                geotag = incoming_data.serialized.clone();
            }

            let source = get_data_source_string(incoming_data.source);

            /**
                TODO: Better setup file IO to write json data from each source into single file for given mission,
                      rather than potentially creating hundreds of different files rapidly

                TODO: Check remaining space on storage drive of destination directory to ensure sufficient storage space
                      and warn on running low
            */
            if incoming_data.serialized.is_some() {
                let serialized = object!{
                    timestamp: incoming_data.time_stamp.to_rfc3339(),
                    data: incoming_data.serialized.unwrap()
                };

                // TODO: ABSTRACT LIVE JSON WRITING TO JSON AS IO STREAM TO MODULE AND FIX
                // Always place ]} at end to maintain file as valid json during write
                let serialized_string = format!("{}\n", serialized.to_string()) + "]}";

                // Seek back by 2 characters in json in order to overwrite } at end that maintains it as a valid fmt
                let overwrite_json_end_offset = serialized_file.stream_position().unwrap() - 2;
                serialized_file.seek(SeekFrom::Start(overwrite_json_end_offset)).expect("TODO: panic message");

                serialized_file.write(serialized_string.as_bytes())
                    .expect(&*format!("Failed to append to file: {source}"));
                // TODO: ABSTRACT LIVE JSON WRITING TO JSON AS IO STREAM TO MODULE AND FIX
            }

            if incoming_data.file.is_some() {
                // if geotag.is_some() {
                //     // TODO: Geotag if file is image
                // }
                //
                // let file = &storage_dir;
                // fs::write(file.clone(), incoming_data.file.unwrap())
                //     .expect(&*format!("Failed to write to file: {}", file));
            }
        }
    }
}
