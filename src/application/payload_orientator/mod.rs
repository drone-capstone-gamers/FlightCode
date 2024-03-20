use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::application::data_manage::{DataSource, IncomingData};
use crate::application::tasks::pib_adapter::PibCommander;

pub fn spawn_payload_orientator(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    thread::spawn(move || {
        payload_orientator_loop(current_data_storage, pib_commander);
    });
}

fn get_drone_orientation(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>) -> f32 {
    let current_attitude_result = &current_data_storage.lock().unwrap()[DataSource::Attitude as usize];

    if current_attitude_result.is_none() || current_attitude_result.as_ref().unwrap().serialized.is_none() {
        return 0.0;
    }

    let json_result = current_attitude_result.as_ref().unwrap().serialized.as_ref().unwrap().to_string();

    // Find the position of the substring
    let substring = "Value:";
    let start_index = match json_result.find(substring) {
        Some(index) => index + substring.len(),
        None => {
            println!("Substring not found");
            return 0.0;
        }
    };

    // Extract the substring containing the float
    let float_str = &json_result[start_index..];

    // Parse the float from the extracted substring
    match float_str.trim().parse::<f32>() {
        Ok(float_value) => {
            println!("Parsed float value: {}", float_value);
            return float_value;
        }
        Err(_) => {
            println!("Failed to parse float");
            return 0.0;
        }
    }
}

const PITCH_ANGLE_MIN: f32 = 0.0;
const PITCH_ANGLE_MAX: f32 = 1.58;

const SERVO_VALUE_MIN: f32 = -128.0;
const SERVO_VALUE_MAX: f32 = 127.0;

fn payload_orientator_loop(current_data_storage: Arc<Mutex<Box<[Option<IncomingData>; DataSource::COUNT]>>>, pib_commander: Arc<PibCommander>) {
    loop {
        let pitch = get_drone_orientation(current_data_storage.clone());

        // 90 degrees downwards(copter) 122 : 90 degrees up (cruise) -128
        // pitch value 0 for copter : pitch value 1.58 for cruise:

        // Perform the linear mapping
        let servo_value = (((pitch - PITCH_ANGLE_MIN) / (PITCH_ANGLE_MAX - PITCH_ANGLE_MIN)) * (SERVO_VALUE_MAX - SERVO_VALUE_MIN) + SERVO_VALUE_MIN) as i8;

        pib_commander.put_servo_set(servo_value);

        sleep(Duration::from_millis(50)); // 20 hz
    }
}
