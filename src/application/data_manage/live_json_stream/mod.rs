use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use json::JsonValue;

pub struct LiveJsonStream {
    first_item_flag: bool,
    file: File
}

impl LiveJsonStream {
    pub fn new(file: File) -> Self {
        let mut instance = Self {
            first_item_flag: true,
            file
        };

        instance.setup();

        return instance;
    }

    fn setup(&mut self) {
        // Setup json file with { at start, && are dummy characters to be removed within write()
        self.file.write("{\"data_stream\": [\n&&".as_bytes())
            .expect("Failed to initialize data capture for unknown reason");
    }

    pub fn write(&mut self, value: JsonValue) {
        let mut comma_delimiter = "";

        if !self.first_item_flag {
            comma_delimiter = ",";
        } else {
            self.first_item_flag = false;
        }

        // Always place ]} at end to maintain file as valid json during write
        let serialized_string = format!("{}\n{}", comma_delimiter, value.to_string()) + "]}";

        // Seek back by 2 characters in json in order to overwrite } at end that maintains it as a valid fmt
        let overwrite_json_end_offset = self.file.stream_position().unwrap() - 2;
        self.file.seek(SeekFrom::Start(overwrite_json_end_offset)).expect("TODO: panic message");

        self.file.write(serialized_string.as_bytes())
            .expect("Failed to append to json stream");
    }
}