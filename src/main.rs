#![windows_subsystem = "windows"]
mod utils;

use utils::{get_local_ip, get_public_address, IPAddr};
use web_view::*;

struct Data{
    port: u32,
    lease_time: u32
}

fn main() {
    // let html_content = "<html><body><h1>Hello, World!</h1></body></html>";
    let html_content = include_str!("pages/index.html");

    web_view::builder()
        .title("Open2Internet - Open Minecraft LAN")
        .content(Content::Html(html_content))
        .size(400, 300)
        .resizable(false)
        .debug(true)
        .user_data(Data{port: 25565, lease_time: 3600})
        .invoke_handler(|webview, arg| {
            match arg {
                "refresh_ips" => {
                    webview
                        .eval(&format!(
                            "document.querySelector(\"#local_ip\").value = \"{}\";",
                            IPAddr::new_ipv4(get_local_ip().expect("Cannot get local IP address!"))
                        ))
                        .expect("Setting the local ip into JS went wrong!");
                    webview
                        .eval(&format!(
                            "document.querySelector(\"#public_ip\").value = \"{}\";",
                            IPAddr::new(
                                get_public_address().expect("Cannot get public IP address!")
                            )
                        ))
                        .expect("Setting the public ip into JS went wrong!");
                }
                "register_port" => {
                    let data = webview.user_data();
                    // let lease_time = webview.eval("document.querySelector(\"#lease_time\")").ok();
                    // let port = webview.eval("document.querySelector(\"#global_port\")").ok();
                    println!("Lease time: {}", data.lease_time);
                    println!("Port: {}", data.port);
                }
                _ => unimplemented!()
            }
            Ok(())
        })
        .build()
        .unwrap()
        .run()
        .unwrap();
}
