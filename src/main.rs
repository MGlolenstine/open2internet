use port_scanner::scan_ports_range;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::io::prelude::*;
use std::time::Duration;
use std::io::{Error};

const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

fn main() {
    let all_ports = 10000..65535;
    let free_ports = scan_ports_range(10000..65535);
    let taken_ports = all_ports.into_iter().filter(|&i|free_ports.contains(&i));
    for open_port in taken_ports {
        if is_minecraft(open_port).is_ok() {
            println!("Port {} is taken", open_port);
        }
    }
}

fn is_minecraft(port: u16) -> Result<bool, Error>{
    let mut stream = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(LOCAL_IP), port), Duration::from_millis(100))?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    let req = [0xFE, 0x01];
    stream.write(&req)?;
    let mut resp = [0u8; 9];
    stream.read(&mut resp)?;
    is_empty(&resp);
    println!("Port {}'s data: {:?}", port, &resp[..]);
    Ok(false)
}

fn is_empty(buffer: &[u8])-> bool{
    for c in buffer {
        if c != 0x0 {
            return false;
        }
    }
    true
}