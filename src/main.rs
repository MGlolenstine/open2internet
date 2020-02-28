use port_scanner::scan_ports_range;
use std::io::prelude::*;
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

fn main() {
    let all_ports = 10000..65535;
    let free_ports = scan_ports_range(10000..65535);
    let taken_ports = all_ports.into_iter().filter(|&i| free_ports.contains(&i));
    for open_port in taken_ports {
        let mcs = is_minecraft(open_port);
        if mcs.is_ok() && mcs.unwrap() {
            println!("Your Minecraft is running on port {}!", open_port);
        }
    }
}

fn is_minecraft(port: u16) -> Result<bool, Error> {
    let mut stream = TcpStream::connect_timeout(
        &SocketAddr::new(IpAddr::V4(LOCAL_IP), port),
        Duration::from_millis(100),
    )?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    let req = [0xFE, 0x01];
    stream.write(&req)?;
    let mut resp = [0u8; 9];
    /*
        If read == [255, 0, 44, 0, 167, 0, 49, 0, 0], it's minecraft server
    */
    stream.read(&mut resp)?;
    //dbg!("Port {}'s data: {:?}", port, &resp[..]);
    if !is_empty(&resp) {
        return Ok(true);
    }
    Ok(false)
}

fn is_empty(buffer: &[u8]) -> bool {
    let mc_server = [255, 0, 44, 0, 167, 0, 49, 0, 0];
    for i in 0..buffer.len() {
        let c = buffer[i];
        if c.clone() != mc_server[i] {
            return true;
        }
    }
    false
}
