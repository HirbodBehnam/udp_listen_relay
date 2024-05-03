use std::{env, net::SocketAddr};

pub struct Arguments {
    /// Initiator is the socket which we should send our first empty packet to.
    pub initiator: SocketAddr,
    /// The other socket
    pub destination: SocketAddr,
}

/// Gets the arguments of the program
pub fn get_arguments() -> Arguments {
    // Get the arguments
    let mut args = env::args();
    args.next(); // skip the program name
    // First argument should be the initiator
    let initiator = args.next().expect("Pass the initiator address as the first argument.");
    let initiator: SocketAddr = initiator.parse().expect("Cannot parse the initiator address.");
    // Second argument should be the destination
    let destination = args.next().expect("Pass the destination address as the second argument.");
    let destination: SocketAddr = destination.parse().expect("Cannot parse the destination address.");
    // Done
    return Arguments { initiator, destination }
}