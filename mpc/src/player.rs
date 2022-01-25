use vm::StoffelVM;
use crate::mpc::protocol;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Player<V: StoffelVM, M: MPCProtocol> {
    id: str,
    vm: V<M>,
    socket: SocketAddr, 
}

impl Player {
    fn new() -> Self {
        todo!();
    }

    fn send_to_player() -> Result<()> {
        todo!();
    }

    fn receive_from_player() -> Result<()> {
        todo!();
    }

    fn broadcast_and_receive() -> Result<()> {
        todo!();
    }

}