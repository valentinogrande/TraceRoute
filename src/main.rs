use std::{io, net::UdpSocket};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:80")?;

    let mut buf: [usize; 1024] = [0; 1024];

    let mut idx = 0;

    let mut url = String::new();

    print!("Introduce tu url: ");
    io::stdin().read_line(&mut url).expect("failed to read url");
    println!();

    loop {
        idx += 1;
        socket.set_ttl(idx)?;
    }

    Ok(())
}
