use dns_lookup::lookup_host;
use std::mem;
use std::{
    io::{self, Write},
    net::UdpSocket,
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:4443")?;

    // Now we will create the ICMP Raw socket via a linux system call.

    let icmp_socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_RAW, libc::IPPROTO_ICMP) };

    let timeout = libc::timeval {
        tv_sec: 1,
        tv_usec: 0,
    };

    // Now we add the timeout for listening packets

    let ret = unsafe {
        libc::setsockopt(
            icmp_socket,
            libc::SOL_SOCKET,
            libc::SO_RCVTIMEO,
            &timeout as *const _ as *const libc::c_void,
            mem::size_of::<libc::timeval>() as libc::socklen_t,
        )
    };

    let mut buf: [u8; 10] = [0; 10];

    let mut idx = 0;

    print!("Introduce tu url: ");
    let _ = io::stdout().flush();
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("failed to read url");

    let port = "12337";
    let url = url.trim();

    let ip = lookup_host(url).unwrap().collect::<Vec<std::net::IpAddr>>()[0];

    println!("The IP Address of {url} is: {ip}");

    loop {
        idx += 1;
        socket.set_ttl(idx)?;

        socket
            .send_to(&buf, format!("{ip}:{port}"))
            .expect("Error sending UDP datagram");

        let now = Instant::now();

        let mut rec_buff = [0; 1024];

        let icmp_bytes = unsafe {
            libc::recv(
                icmp_socket,
                rec_buff.as_mut_ptr() as *mut libc::c_void,
                rec_buff.len(),
                0,
            )
        };

        if icmp_bytes < 0 {
            break;
        }

        let delay = (Instant::now() - now).as_millis();

        let packet =
            etherparse::SlicedPacket::from_ip(&rec_buff[..icmp_bytes as usize]).expect("Error");

        let router_ip = packet
            .net
            .unwrap()
            .ipv4_ref()
            .unwrap()
            .header()
            .source_addr();

        println!("[*] ROUTER IP: {router_ip} TOTAL DELAY: {delay}ms");

        if idx == 30 {
            break;
        }
    }

    unsafe { libc::close(icmp_socket) };

    Ok(())
}
