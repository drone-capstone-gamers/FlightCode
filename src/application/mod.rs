use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::time::Duration;
use crate::application::tasks::capture_go_pro_images::GoProTask;
use crate::application::data_manage::{IncomingData, spawn_data_manager};
use crate::application::tasks::capture_ircam_images::CaptureIrImages;
use crate::application::tasks::example_task::ExampleTask;
use crate::application::tasks::pib_adapter::{PibAdapter, PibCommander};
use crate::application::tasks::mavlink_adapter::MavlinkAdapter;
use crate::application::timer::{spawn_timer, Timer};

mod timer;
mod tasks;
mod data_manage;

pub trait DataCollector {
    fn new(storage_sender: SyncSender<IncomingData>) -> Self;
}

pub fn start_application() {
    let (queue_sender, queue_recv) = mpsc::sync_channel(10);

    spawn_data_manager(queue_recv);

    let example_task = ExampleTask::new(queue_sender.clone());
    let example_timer = Timer::new("Example_Task".to_string(), Duration::from_secs(1));
    let example_timer_handler = spawn_timer(example_timer, Box::from(example_task));

    let example_task1 = ExampleTask::new(queue_sender.clone());
    let example_timer1 = Timer::new("Example_Task1".to_string(), Duration::from_secs(2));
    let example_timer1_handler = spawn_timer(example_timer1, Box::from(example_task1));

    let gopro_task = GoProTask::new();
    let gopro_timer = Timer::new("GoProControl".to_string(), Duration::from_secs(5));
    let gopro_handler = spawn_timer(gopro_timer, Box::from(gopro_task));

    let ir_cam_task = CaptureIrImages::new(queue_sender.clone());
    let ir_cam_timer = Timer::new("GoProControl".to_string(), Duration::from_secs(5));
    let ir_cam_handler = spawn_timer(ir_cam_timer, Box::from(ir_cam_task));

    let (frame_sender, frame_recv) = mpsc::sync_channel(10);
    let pib_adapter_task = PibAdapter::new(queue_sender.clone(), frame_recv);
    let pib_adapter_timer = Timer::new("PIBAdapter".to_string(), Duration::from_secs(1));
    let pib_adapter_handler = spawn_timer(pib_adapter_timer, Box::from(pib_adapter_task));

    let mavlink_adapter = MavlinkAdapter::new(queue_sender.clone());
    let mavlink_adapter_timer = Timer::new("MavlinkAdapter".to_string(), Duration::from_millis(100));
    let mavlink_adapter_handler = spawn_timer(mavlink_adapter_timer, Box::from(mavlink_adapter));

    let pib_commander = PibCommander::new(frame_sender);
    pib_commander.get_temperature_request();

    let (ctrlc_tx, ctrlc_rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        example_timer_handler.send(true).expect("Failed to send kill signal to collection task");
        example_timer1_handler.send(true).expect("Failed to send kill signal to collection task");

        gopro_handler.send(true).expect("Failed to send kill signal to collection task");
        ir_cam_handler.send(true).expect("Failed to send kill signal to collection task");
        pib_adapter_handler.send(true).expect("Failed to send kill signal to collection task");
        mavlink_adapter_handler.send(true).expect("Failed to send kill signal to collection task");

        ctrlc_tx.send(true).expect("Failed to send signal to shutdown main thread!");
    }).expect("Error setting Ctrl-C handler");

    let mut must_quit = false;
    while !must_quit {
        let contr_shutdown = ctrlc_rx.recv_timeout(Duration::from_millis(100));
        if contr_shutdown.is_ok() {
            if contr_shutdown.unwrap() {
                must_quit = true;
            }
        }
    }
}
