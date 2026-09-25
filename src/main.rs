use etherparse::{Ipv4Header, Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice};
use regex::Regex;
use std::{net::Ipv4Addr, process::Command};
use tun_tap::Iface;

pub fn set_interface() -> Result<Iface, std::io::Error> {
    let interface = Iface::new("tun0", tun_tap::Mode::Tap)?;

    //setting ip for interface Iface0

    let ip_address = "10.0.0.1"; // private ip
    let mask = "24";
    let ip = format!("{}/{}", ip_address, mask);

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(interface.name())
        .spawn()
        .expect("Failed to set ip")
        .wait();

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(interface.name())
        .arg("up")
        .spawn()
        .expect("Error setting interface up")
        .wait();

    Ok(interface)
}

fn main() {
    let interface = set_interface().unwrap();

    let url = "google.com";

    let ping = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(url)
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&ping.stdout);

    let re = Regex::new(r"\((\d{1,3}(?:\.\d{1,3}){3})\)").unwrap();

    let mut ip = String::new();
    if let Some(caps) = re.captures(&stdout) {
        ip = caps[1].to_string();
        println!("{}", ip);
    }

    let destination: Ipv4Addr = ip.parse().unwrap();
    let source: Ipv4Addr = "10.0.0.1".parse().unwrap();

    let protocol = etherparse::IpNumber::UDP;
    let time_to_live = 1;

    let total_len = Ipv4Header::MIN_LEN;

    let trace = Ipv4Header::new(
        time_to_live,
        total_len as u8,
        protocol,
        destination.octets(),
        source.octets(),
    );

    let mut buffer: [u8; Ipv4Header::MIN_LEN] = [0u8; Ipv4Header::MIN_LEN];

    let s = {
        let mut s = &mut buffer[..];

        trace.unwrap().write(&mut s).unwrap();
        s.len()
    };

    let a = interface.send(&buffer[..s]);

    if let Err(e) = a {
        println!("{}", e);
    }
}
