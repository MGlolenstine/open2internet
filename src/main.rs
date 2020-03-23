#![windows_subsystem = "windows"]
mod utils;
mod ui;

use web_view::*;

fn main() {
    // let html_content = "<html><body><h1>Hello, World!</h1></body></html>";
    let html_content = include_str!("pages/index.html");
	
    web_view::builder()
        .title("Open2Internet - Open Minecraft LAN")
        .content(Content::Html(html_content))
        .size(640, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap()
        .run()
        .unwrap();
}
