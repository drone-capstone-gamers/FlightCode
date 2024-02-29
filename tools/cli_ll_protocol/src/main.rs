extern crate ll_protocol;

use std::{env, io, thread};
use std::sync::mpsc;
use std::time::Duration;
use ll_protocol::frame::Frame;
use ll_protocol::frame_deserializer;
use ll_protocol::frame_serializer::FrameSerializer;
use serialport::SerialPort;

fn decode_hex_string_into_payload(hex_string: &str) -> Vec<u8> {
    let bytes: Vec<u8> = hex::decode(hex_string).unwrap_or_else(|err| {
        eprintln!("Error decoding hex string: {}", err);
        vec![]
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

    // Handler for user exit via keyboard interrupt
    let (ctrlc_tx, ctrlc_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        println!("received Ctrl+C, quitting!");
        ctrlc_tx.send(true).expect("Failed to send signal to shutdown main thread!");
    }).expect("Error setting Ctrl-C handler");

    let (send_frame_tx, send_frame_rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    let service = input.remove(0).to_string().parse::<u8>().unwrap();
                    let hex_string = &*input.replace("0x", "")
                        .replace(" ", "")
                        .replace(",", "")
                        .replace("\n", "");

                    let frame = Frame::new(service, decode_hex_string_into_payload(hex_string));

                    send_frame_tx.send(frame).expect("Failed to send frame to main thread!");
                }
                Err(error) => println!("error: {error}"),
            }
        }
    });

    let mut must_quit = false;
    while !must_quit {
        let bytes_to_read = port.bytes_to_read().unwrap();

        if bytes_to_read > 0 {
            port.read(serial_buf.as_mut_slice()).expect("Found no data!");

            serial_buf.iter().map(|&byte| frame_deserializer.apply(byte))
                .filter(|result| result.is_some())
                .for_each(|result| {
                    let deserialized_frame = result.unwrap();
                    println!("\n\nReceived frame: {}", deserialized_frame);
                });
        }

        let contr_shutdown = ctrlc_rx.recv_timeout(Duration::from_millis(100));
        if contr_shutdown.is_ok() {
            if contr_shutdown.unwrap() {
                must_quit = true;
            }
        }

        let send_frame_result = send_frame_rx.recv_timeout(Duration::from_millis(100));
        if send_frame_result.is_ok() {
            let frame = send_frame_result.unwrap();

            // TODO: factor out duplicate code from send_frame()
            let frame_serializer = FrameSerializer::new(frame.clone(), true);

            let serialized_frame =  &frame_serializer.collect::<Vec<u8>>();

            port.write(serialized_frame).expect("Write failed!");

            println!("\n\nSent frame: {}", frame);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        eprintln!("Usage: {} [send <port> <service> <hex_value1> <hex_value2> ... | receive <port>]", args[0]);
        std::process::exit(1);
    }

    let port = serialport::new(&args[2], 9_600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    match args[1].as_str() {
        "send" => {
            if args.len() <= 4 {
                eprintln!("Usage: {} send <port> <service> <hex_value1> <hex_value2> ...", args[0]);
                std::process::exit(1);
            }

            let mut args_copy = args;

            let service = args_copy[3].parse().unwrap();

            args_copy.remove(0);args_copy.remove(0);args_copy.remove(0);args_copy.remove(0);

            let hex_string = &*args_copy.join("").replace("0x", "");

            let frame = Frame::new(service, decode_hex_string_into_payload(hex_string));

            send_frame(port, frame);
        }
        "receive" => {
            receive_frames(port);
        }
        _ => {
            eprintln!("Usage: {} [send <port> <service> <hex_value1> <hex_value2> ... | receive <port>]", args[0]);
            std::process::exit(1);
        }
    }
}
