#![allow(dead_code)]

use crate::error::{Error, Result};
use get_if_addrs::get_if_addrs;
use netstat::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::time::Duration;

const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const WANTED_PORT: u16 = 25565;

/*
    Get host's local IP.
*/
pub fn get_local_ip() -> Result<Ipv4Addr> {
    for iface in get_if_addrs()? {
        let ip = iface.ip().to_string();
        /*
            Filter out local IPs for IPv4 and IPv6, and also Docker IPs.
            Also, UPnP doesn't support IPv6 forwarding, so we can just remove all IPv6 addresses.
        */
        if !iface.is_loopback() && iface.ip().is_ipv4() && !ip.starts_with("172.") {
            match iface.ip() {
                IpAddr::V4(ip) => return Ok(ip),
                _ => continue,
            }
        }
    }
    return Err(Error::NoIPv4Found);
}

/*
    Scan ports in Minecraft range, to find the Minecraft one.
*/
pub fn scan_ports() -> Result<Vec<u16>> {
    Ok(get_used_ports()?
        .into_iter()
        .filter(|v| is_minecraft(*v).unwrap_or(false))
        .collect())
}

pub fn get_used_ports() -> Result<Vec<u16>> {
    let af_flags = AddressFamilyFlags::all();
    let proto_flags = ProtocolFlags::all();
    let sockets_info = get_sockets_info(af_flags, proto_flags)?;
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
    Ok(ports)
}

/*
    Check if the port at the local address is responding with correct data for Minecraft server.
*/
fn is_minecraft(port: u16) -> Result<bool> {
    let mut stream = TcpStream::connect_timeout(
        &SocketAddr::new(IpAddr::V4(LOCAL_IP), port),
        Duration::from_millis(10),
    )?;
    stream.set_read_timeout(Some(Duration::from_millis(10)))?;
    let req = [0xFE, 0x01];
    stream.write(&req)?;
    let mut resp = [0u8; 18];
    /*
        If read == [255, 0, 44, 0, 167, 0, 49, 0, 0], it's minecraft server
    */
    let buf_len = stream.read(&mut resp);
    if buf_len.is_err() {
        return Ok(false);
    }
    if is_minecraft_response(&resp) {
        return Ok(true);
    }
    Ok(false)
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
pub fn get_public_address() -> Result<IpAddr> {
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

    // Attempt to get an IP address and print it
    // if let Some(ip) = public_ip::addr().await {
    //     println!("public ip address: {:?}", ip);
    //     return Some(ip);
    // } else {
    //     println!("couldn't get an IP address");
    // }
    let ip = public_ip_address::perform_lookup(None).map(|a| a.ip)?;
    Ok(ip)
}

/*
    Uses UPnP to port-forward the automagically generated port to the defined external one.
*/
pub fn redirect_minecraft_to_a_port(mc_port: u16, wanted_port: u16, lease: u32) -> Result<()> {
    let local_addr = get_local_ip()?;
    // UPnP only works on local IPv4 addresses
    let local_addr = SocketAddrV4::new(local_addr, mc_port);
    let gateway = igd::search_gateway(Default::default())?;
    gateway.add_port(
        igd::PortMappingProtocol::TCP,
        wanted_port,
        local_addr.into(),
        lease,
        "MinecraftLAN",
    )?;

    Ok(())
}

pub fn remove_minecraft_redirect(wanted_port: u16) -> Result<()> {
    let gateway = igd::search_gateway(Default::default())?;
    Ok(gateway.remove_port(igd::PortMappingProtocol::TCP, wanted_port)?)
}
