#![windows_subsystem = "windows"]
mod ui;
mod utils;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use std::net::IpAddr;
use std::sync::Arc;
use ui::ui_builder;
use ui::O2IInfo;
use utils::{get_local_ip, get_public_address, scan_ports};

fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("open-2-internet").with_placeholder("Open 2 Internet"));
    let data = O2IInfo{
        local_address: IpAddr::V4(get_local_ip().expect("Something went wrong receiving the local IP address!")),
        public_address: get_public_address().expect("Something went wrong receiving the public IP address!\nAre you connected to the internet?"),
        ports: Arc::new(scan_ports()),
    };
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}
