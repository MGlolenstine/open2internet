#![allow(dead_code)]

use port_scanner::{local_ports_available_range, scan_port};
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::time::Duration;
// use natpmp::*;
use get_if_addrs::get_if_addrs;
use igd::search_gateway;
// use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Range;
use std::sync::mpsc::channel;
use std::thread;

const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const WANTED_PORT: u16 = 25565;

/*
    Get host's local IP.
*/
pub fn get_local_ip() -> Option<Ipv4Addr> {
    for iface in get_if_addrs().unwrap() {
        let ip = iface.ip().to_string();
        /*
            Filter out local IPs for IPv4 and IPv6, and also Docker IPs.
            Also, UPnP doesn't support IPv6 forwarding, so we can just remove all IPv6 addresses.
        */
        if !iface.is_loopback() && iface.ip().is_ipv4() && !ip.starts_with("172.") {
            let splot = ip.split(".").collect::<Vec<&str>>();
            return Some(Ipv4Addr::new(
                splot[0].parse::<u8>().unwrap(),
                splot[1].parse::<u8>().unwrap(),
                splot[2].parse::<u8>().unwrap(),
                splot[3].parse::<u8>().unwrap(),
            ));
            // return Some(iface.ip());
        }
    }
    return None;
}

/*
    Scan ports in Minecraft range, to find the Minecraft one.
*/
pub fn scan_ports() -> Vec<u16> {
    let mut vec: Vec<u16> = Vec::new();
    let all_ports = 10000..65535;
    let free_ports = HashSet::<u16>::from_iter(
        local_ports_available_range(all_ports.clone())
            .iter()
            .cloned(),
    );
    let taken_ports: Vec<u16> = all_ports.filter(|&i| !free_ports.contains(&i)).collect();
    let (sender, receiver) = channel();
    for open_port in taken_ports.clone() {
        let local_sender = sender.clone();
        thread::spawn(move || {
            local_sender
                .send(if is_minecraft(open_port) {
                    Some(open_port)
                } else {
                    None
                })
                .unwrap()
        });
    }
    for _ in &taken_ports {
        let rec = receiver.recv().unwrap();
        if rec.is_some() {
            vec.push(rec.unwrap());
        }
    }
    return vec;
}

pub fn scan_ports_range(port_range: Range<u16>) -> Vec<u16> {
    let mut open_ports = Vec::new();
    let (sender, receiver) = channel();
    for port in port_range.clone() {
        let local_sender = sender.clone();
        thread::spawn(move || {
            local_sender
                .send(if scan_port(port) { Some(port) } else { None })
                .unwrap();
        });
    }
    for _ in port_range {
        let rec = receiver.recv().unwrap();
        if rec.is_some() {
            open_ports.push(rec.unwrap());
        }
    }
    open_ports
}

/*
    Check if the port at the local address is responding with correct data for Minecraft server.
*/
pub fn is_minecraft(port: u16) -> bool {
    let stream = TcpStream::connect_timeout(
        &SocketAddr::new(IpAddr::V4(LOCAL_IP), port),
        Duration::from_millis(10),
    );
    if stream.is_err() {
        return false;
    }
    let mut stream = stream.unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    let req = [0xFE, 0x01];
    stream.write(&req).unwrap();
    let mut resp = [0u8; 18];
    /*
        If read == [255, 0, 44, 0, 167, 0, 49, 0, 0], it's minecraft server
    */
    let buf_len = stream.read(&mut resp);
    if buf_len.is_err() {
        return false;
    }
    if is_minecraft_response(&resp) {
        return true;
    }
    false
}

/*
    Check if the returned buffer equals to Minecraft's response.
*/
pub fn is_minecraft_response(buffer: &[u8]) -> bool {
    //let mc_server = [255, 0, 44, 0, 167, 0, 49, 0, 0];
    /*
        Protocol explanation
        [
            255 -> FF to denote it's a MC server
            0, 44  -> length
            0, 167 -> §
            0, 49  -> 1
            0,0    -> padding
            0, 49  -> 1
            0, 50  -> 2
            0, 55  -> 7
            ...
        ]
    */
    let mc_server = [
        255, 0, 42, 0, 167, 0, 49, 0, 0, 0, 49, 0, 50, 0, 55, 0, 0, 0,
    ];
    for i in 0..buffer.len() {
        if i == 2 {
            continue;
        }
        let c = buffer[i];
        if c.clone() != mc_server[i] {
            return false;
        }
    }
    true
}

/*
    Get host's external IP address, to give them IP for their friends to join.
*/
pub fn get_public_address() -> Option<IpAddr> {
    let gateway = search_gateway(Default::default()).unwrap();
    let ip = IpAddr::V4(gateway.get_external_ip().unwrap());
    return Some(ip);
    // None
}

/*
    Uses UPnP to port-forward the automagically generated port to the defined external one.
*/
pub fn redirect_minecraft_to_a_port(mc_port: u16, wanted_port: u16, lease: u32) {
    let local_addr = get_local_ip().unwrap();
    // UPnP only works on local IPv4 addresses
    let local_addr = SocketAddrV4::new(local_addr, mc_port);
    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error finding gateway: {}", err),
        Ok(gateway) => {
            // println!(
            //     "gateway.add_port({},{},{},{},{})",
            //     igd::PortMappingProtocol::TCP,
            //     wanted_port,
            //     local_addr.to_string(),
            //     lease,
            //     "MinecraftLAN"
            // );
            match gateway.add_port(
                igd::PortMappingProtocol::TCP,
                wanted_port,
                local_addr.into(),
                lease,
                "MinecraftLAN",
            ) {
                Err(ref err) => {
                    println!("There was an error registering the port! {}", err);
                }
                Ok(_) => {
                    println!("It worked! Got port {}, hopefully!", wanted_port);
                }
            }
        }
    }
}
