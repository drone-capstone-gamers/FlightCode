extern crate ll_protocol;

use std::env;
use std::time::Duration;
use ll_protocol::frame::Frame;
use ll_protocol::frame_deserializer;
use ll_protocol::frame_serializer::FrameSerializer;
use serialport::SerialPort;

fn decode_hex_string_into_payload(hex_string: &str) -> Vec<u8> {
    let bytes: Vec<u8> = hex::decode(hex_string).unwrap_or_else(|err| {
        eprintln!("Error decoding hex string: {}", err);
        std::process::exit(1);
    });

    return bytes;
}

fn send_frame(mut port: Box<dyn SerialPort>, frame: Frame) {
    let frame_serializer = FrameSerializer::new(frame.clone(), true);

    let serialized_frame =  &frame_serializer.collect::<Vec<u8>>();

    port.write(serialized_frame).expect("Write failed!");

    print!("\n\nSent frame: {}", frame);
}

fn receive_frames(mut port: Box<dyn SerialPort>) {
    let mut serial_buf: Vec<u8> = vec![0; 128];
    let mut frame_deserializer = frame_deserializer::FrameDeserializer::new();

    loop {
        let bytes_to_read = port.bytes_to_read().unwrap();

        if bytes_to_read > 0 {
            port.read(serial_buf.as_mut_slice()).expect("Found no data!");

            serial_buf.iter().map(|&byte| frame_deserializer.apply(byte))
                .filter(|result| result.is_some())
                .for_each(|result| {
                    let deserialized_frame = result.unwrap();
                    print!("\n\nReceived frame: {}", deserialized_frame);
                });
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Usage: {} [send <service> <hex_value1> <hex_value2> ... | receive]", args[0]);
        std::process::exit(1);
    }

    // TODO: CLOSE THE PORT UPON CLOSE OF PROGRAM
    let port = serialport::new("/dev/pts/4", 115_200) // TODO: pass port as arg
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");
    match args[1].as_str() {
        "send" => {
            if args.len() <= 3 {
                eprintln!("Usage: {} send <service> <hex_value1> <hex_value2> ...", args[0]);
                std::process::exit(1);
            }

            let mut args_copy = args;

            let service = args_copy[2].parse().unwrap();

            args_copy.remove(0);args_copy.remove(0);args_copy.remove(0);

            let hex_string = &*args_copy.join("").replace("0x", "");

            let frame = Frame::new(service, decode_hex_string_into_payload(hex_string));

            send_frame(port, frame);
        }
        "receive" => {
            receive_frames(port);
        }
        _ => {
            eprintln!("Usage: {} [send <service> <hex_value1> <hex_value2> ... | receive]", args[0]);
            std::process::exit(1);
        }
    }
}
