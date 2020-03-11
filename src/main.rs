#![windows_subsystem = "windows"]
// mod ui;
mod utils;
mod rust_ui;
// mod style;

// use std::net::IpAddr;
// use std::sync::Arc;
// use ui::O2IInfo;
use rust_ui::Styling;
use iced::Settings;
use iced::{Sandbox};
// use utils::{get_local_ip, get_public_address, scan_ports};

fn main() {
    Styling::run(Settings::default());
}
