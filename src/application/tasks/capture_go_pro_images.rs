use reqwest::blocking::Client;
use envconfig::Envconfig;
use crate::application::timer::TimedTask;

#[derive(Envconfig)]
struct GoProConfig {
    #[envconfig(from = "GOPRO_HTTP_ADDR", default = "http://172.23.186.51:8080")]
    pub go_pro_addr: String
}

pub struct GoProTask {
    client: Client,
    go_pro_addr: String
}

impl GoProTask {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            go_pro_addr: GoProConfig::init_from_env().unwrap().go_pro_addr
        }
    }
}

impl TimedTask for GoProTask {
    fn execute(&mut self) -> () {
        let response = self.client.get(&*format!("{}/gopro/camera/keep_alive", self.go_pro_addr)).send();

        if response.is_err() || response.unwrap().status() != 200 {
            println!("WARNING: GoPro is not responding to keep alive command");
            return;
        }
    }
}