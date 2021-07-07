#![allow(dead_code)]

use get_if_addrs::get_if_addrs;
use igd::search_gateway;
use netstat::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use public_ip::ToResolver;
use public_ip::{dns, http, BoxToResolver};
use std::fmt::Display;
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::time::Duration;

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
    let all_ports = get_used_ports();

    let ports: Vec<_> = all_ports
        .iter()
        .map(|&v| if is_minecraft(v) { v } else { 0 })
        .filter(|&x| x != 0)
        .collect::<Vec<_>>();
    return ports;
}

pub fn get_used_ports() -> Vec<u16> {
    let af_flags = AddressFamilyFlags::all();
    let proto_flags = ProtocolFlags::all();
    let sockets_info = get_sockets_info(af_flags, proto_flags).unwrap();
    let mut ports = vec![];
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp_si) => match tcp_si.state {
                netstat::TcpState::Listen => {
                    ports.push(tcp_si.local_port);
                }
                _ => {}
            },
            _ => {}
        }
    }
    ports
}

/*
    Check if the port at the local address is responding with correct data for Minecraft server.
*/
fn is_minecraft(port: u16) -> bool {
    let stream = TcpStream::connect_timeout(
        &SocketAddr::new(IpAddr::V4(LOCAL_IP), port),
        Duration::from_millis(10),
    );
    if stream.is_err() {
        return false;
    }
    let mut stream = stream.unwrap();
    stream
        .set_read_timeout(Some(Duration::from_millis(10)))
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
fn is_minecraft_response(buffer: &[u8]) -> bool {
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
    // let mc_server = [
    //     255, 0, 42, 0, 167, 0, 49, 0, 0, 0, 49, 0, 50, 0, 55, 0, 0, 0,
    // ];
    buffer[0] == 255 && buffer[1] == 0
    // for i in 0..buffer.len() {
    //     if i == 2 {
    //         continue;
    //     }
    //     let c = buffer[i];
    //     if c.clone() != mc_server[i] {
    //         return false;
    //     }
    // }
    // true
}

/*
    Get host's external IP address, to give them IP for their friends to join.
*/
pub async fn get_public_address() -> Option<IpAddr> {
    // Startup on Windows takes ages if this isn't commented.
    // let gtw = search_gateway(Default::default());
    // if let Ok(gateway) = gtw {
    //     let ip = IpAddr::V4(gateway.get_external_ip().unwrap());
    //     return Some(ip);
    // }
    // println!(
    //     "The IP is either IPv6 or some other error occured: {}",
    //     gtw.unwrap_err()
    // );
    let resolver = vec![
        BoxToResolver::new(dns::OPENDNS_RESOLVER),
        BoxToResolver::new(http::HTTP_IPIFY_ORG_RESOLVER),
    ]
    .to_resolver();
    // Attempt to get an IP address and print it
    if let Some(ip) = public_ip::resolve_address(resolver).await {
        println!("public ip address: {:?}", ip);
        return Some(ip);
    } else {
        println!("couldn't get an IP address");
    }
    println!("Keep in mind, that having an IPv6 address as the server address, only people with IPv6 will be able to join!");
    return None;
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

pub struct IPAddr {
    ip_addr: IpAddr,
}

impl IPAddr {
    pub fn new_ipv4(a: Ipv4Addr) -> IPAddr {
        IPAddr {
            ip_addr: IpAddr::V4(a),
        }
    }

    pub fn new(a: IpAddr) -> IPAddr {
        IPAddr { ip_addr: a }
    }
}

impl Display for IPAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ip_addr {
            IpAddr::V4(v) => {
                let octets = v.octets();
                std::write!(f, "{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
            }
            IpAddr::V6(_v) => {
                std::write!(f, "IPv6 - Check IPv4 address!")
                // let octets = v.octets();
                // std::write!(f, "{:02X}{:02x}:{:02x}{:02x}:{:02X}{:02x}:{:02x}{:02x}:{:02X}{:02x}:{:02x}{:02x}:{:02X}{:02x}:{:02x}{:02x}", octets[0], octets[1], octets[2], octets[3], octets[4], octets[5], octets[6], octets[7], octets[8], octets[9], octets[10], octets[11], octets[12], octets[13], octets[14], octets[15])
            }
        }
    }
}
