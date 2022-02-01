use super::protocols::{MPCProtocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Player {
    id: String,
    socket: SocketAddr,
    protocol: MPCProtocol,
    vm: StoffelVM,
}

impl Player {
    fn new() -> Self {
        todo!();
    }

    fn send_to_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn receive_from_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn broadcast_and_receive() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

}