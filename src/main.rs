#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
use std::{thread, time::Duration};

use application::logging::start_logger;
use dotenv::dotenv;
use flexi_logger::LoggerHandle;
use network::{
    models::{Board, FieldState},
    socket::init_connection,
};

mod application;
mod game;
mod network;
pub mod utils;

fn main() {
    dotenv().ok();
    let logger_handle = start_logger();

    init_connection();

    loop {
        thread::sleep(Duration::from_secs(60))
    }
}
