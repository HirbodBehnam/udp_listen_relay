use std::net::UdpSocket;

use log::{debug, info, warn};

mod arguments;

fn main() {
    env_logger::init();
    let args = arguments::get_arguments();
    // Create the sockets
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Cannot bind a random UDP port");
    debug!("Local socket bind on {}", socket.local_addr().unwrap());
    // Start relaying by sending an empty packet
    socket
        .send_to(&[], args.initiator)
        .expect("Cannot send the first packet to initiator");
    info!("Relaying packets...");
    // Wait for a packet...
    let mut buffer = [0u8; 1024 * 4];
    loop {
        let (n, src) = socket.recv_from(&mut buffer).expect("Cannot receive data");
        // Where is the source of this packet?
        if src == args.initiator {
            // Send packet to destination
            socket
                .send_to(&buffer[..n], args.destination)
                .expect("Cannot send packet to destination");
        } else if src == args.destination {
            // Send packet to initiator
            socket
                .send_to(&buffer[..n], args.initiator)
                .expect("Cannot send packet to initiator");
        } else {
            // WTF?
            warn!("Unknown packet source: {}", src);
        }
    }
}
