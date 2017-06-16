use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use std::string::String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub motors: Vec<u32>,
    pub motor_cutoff: f32,
    pub sensor_poll_time: i64,
    pub motors_on: bool,
    pub integral_decay_time: f32,
    pub server_address: String,
    pub hover_power: u32,
    pub debug_websocket_port: i32,
}

impl Config {
    pub fn new() -> Config {
        let mut file = File::open("config.json").expect("failed to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read config.json");
        // Parse the string of data into serde_json::Value.
        let v: Config = serde_json::from_str(&contents).expect("failed to desrialize json");
        v
    }
}