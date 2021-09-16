use std::net::{SocketAddr, UdpSocket};

use log::{trace, warn};

use crate::control::parser;

pub fn run(socket_addrs: &[SocketAddr]) -> std::io::Result<()> {
    let socket_addrs: Vec<SocketAddr> = socket_addrs
        .iter()
        .map(|addr| SocketAddr::new(addr.ip(), 38913))
        .collect();

    std::thread::spawn(move || {
        let socket = UdpSocket::bind(&*socket_addrs).unwrap();
        let mut buffer = [0; 1024];

        loop {
            let (size, source) = socket.recv_from(&mut buffer).unwrap();

            let buffer = &mut buffer[..size];
            let text = std::str::from_utf8(buffer).unwrap();
            let command = match parser::parse_command(&text) {
                Ok(v) => v,
                Err(e) => {
                    let error = format!("Error while processing command: {}", e);
                    warn!("{:?}", error);
                    socket.send_to(error.as_bytes(), source).unwrap();
                    return;
                }
            };
            trace!("{:?}", command);
            command.execute();
        }
    });

    Ok(())
}
