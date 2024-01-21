use std::sync::mpsc::Receiver;
use std::{fs, thread};
use std::path::Path;
use std::time::Duration;
use chrono::{DateTime, Local};
use json::JsonValue;

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

pub fn spawn_data_manager(data_receiver: Receiver<IncomingData>) {
    thread::spawn(|| {
        data_manager_loop(data_receiver);
    });
}

fn data_manager_loop(data_receiver: Receiver<IncomingData>) {
    let mut geotag: Option<JsonValue> = None;

    loop {
        let data_result = data_receiver.recv_timeout(Duration::from_millis(1));
        if data_result.is_ok() {
            let incoming_data = data_result.unwrap();

            // Record GPS coords into local variable to use for geotagging images
            if incoming_data.source == DataSource::Gps {
                geotag = incoming_data.serialized.clone();
            }

            let source = get_data_source_string(incoming_data.source);

            if !Path::new(&source).exists() {
                fs::create_dir(source.clone()).expect(&*format!("Failed to create directory: {}", source.clone()));

            }

            let full_path = format!("{}/{}", source, &*incoming_data.time_stamp.to_rfc3339());

            /**
                TODO: Better setup file IO to write json data from each source into single file for given mission,
                      rather than potentially creating hundreds of different files rapidly
            */
            if incoming_data.serialized.is_some() {
                let file = full_path.clone() + ".json";
                fs::write(file.clone(), incoming_data.serialized.unwrap().to_string())
                    .expect(&*format!("Failed to write to file: {}", file));
            }

            if incoming_data.file.is_some() {
                if geotag.is_some() {
                    // TODO: Geotag if file is image
                }

                let file = full_path.clone();
                fs::write(file.clone(), incoming_data.file.unwrap())
                    .expect(&*format!("Failed to write to file: {}", file));
            }
        }
    }
}
