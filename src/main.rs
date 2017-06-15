extern crate time;

extern crate rust_pigpio;
extern crate sensors;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate websocket;
extern crate protobuf;

use std::io::stdin;
use std::string::String;

mod motor;
use motor::MotorManager;

mod proto {
    pub mod position;
}

mod debug;

use protobuf::Message;
use proto::position::Position;

mod connection;
use connection::Peer;

mod config;
use config::Config;

fn main() {

    let config = Config::new();

//    let mut peer = Peer::new();
    let mut manager = MotorManager::new();
    manager.new_motor(config.motors[0]);
    manager.new_motor(config.motors[1]);
    manager.new_motor(config.motors[2]);
    manager.new_motor(config.motors[3]);

    println!("Press enter to self control.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");

    if config.motors_on {
        manager.arm();
    }
    manager.start_pid_loop(config);

    'input: loop {
        println!("Enter 'stop'");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error");

        match input.trim() {
            "stop" => {
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input;
            },
            _ => {
                println!("unrecognized input...");
                motor::TERMINATE_ALL_MOTORS();
                std::process::exit(0);
                break 'input;
            }
        }
    }
}

const FLOATING_POWER : u32 = 1100;
const MAX_POWER : u32 = 1400;

