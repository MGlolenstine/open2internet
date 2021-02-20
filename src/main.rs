#![windows_subsystem = "windows"]
mod utils;

use serde::{Deserialize, Serialize};
use utils::scan_ports;
use utils::*;
use web_view::Content;

#[derive(Serialize, Deserialize)]
struct Data {
    action: String,
    port: Option<u16>,
    lease_time: Option<u32>,
    selected_port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let html_content = include_str!("pages/index.html");
    let public_ip = dbg!(get_public_address().await);
    dbg!(scan_ports());
    web_view::builder()
        .title("Open2Internet - Open Minecraft LAN")
        .content(Content::Html(html_content))
        .size(400, 300)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            let data: Data = serde_json::from_str(arg).unwrap();
            if data.action == "refresh_ips".to_string() {
                webview
                    .eval(&format!(
                        "document.querySelector(\"#local_ip\").value = \"{}\";",
                        IPAddr::new_ipv4(get_local_ip().expect("Cannot get local IP address!"))
                    ))
                    .expect("Setting the local ip into JS went wrong!");
                if let Some(pa) = public_ip {
                    webview
                        .eval(&format!(
                            "document.querySelector(\"#public_ip\").value = \"{}\";",
                            IPAddr::new(pa)
                        ))
                        .expect("Setting the public ip into JS went wrong!");
                } else {
                    webview
                        .eval(&format!(
                            "document.querySelector(\"#public_ip\").value = \"{}\";",
                            "Error: {Maybe IPv6}"
                        ))
                        .expect("Setting the public ip into JS went wrong!");
                }
            } else if data.action == "register_port".to_string() {
                redirect_minecraft_to_a_port(
                    data.selected_port.unwrap(),
                    data.port.unwrap(),
                    data.lease_time.unwrap(),
                );
            } else if data.action == "refresh_ports".to_string() {
                let ports = scan_ports();
                webview
                    .eval("clear_ports()")
                    .expect("Setting the local ip into JS went wrong!");
                if ports.len() == 0 {
                    webview
                        .eval(&format!(
                            "add_port(\"{}\", {});",
                            "No Minecraft instances found", "-1"
                        ))
                        .expect("Setting the \"No Minecraft instances\" into JS went wrong!");
                } else {
                    for p in ports {
                        let text = format!("Minecraft on port {}", p);
                        webview
                            .eval(&format!("add_port(\"{}\", {});", text, p))
                            .expect("Setting the port into JS went wrong!");
                    }
                }
            }
            Ok(())
        })
        .build()
        .unwrap()
        .run()
        .unwrap();
}
