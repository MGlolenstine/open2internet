mod ui;
mod utils;
use utils::{get_local_ip, get_public_address, scan_ports};
use std::net::IpAddr;

fn main() {
    ui::open_main_window(IpAddr::V4(get_local_ip().unwrap()), get_public_address().unwrap(), scan_ports());
}