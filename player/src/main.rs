use std::net::SocketAddr;

pub struct Player {
    id: String,
    socket: SocketAddr,
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

fn main() {
    println!("Hello, world!");
}
