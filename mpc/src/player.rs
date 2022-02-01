<<<<<<< HEAD
use super::protocols::{MPCProtocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Player {
    id: String,
    socket: SocketAddr,
    protocol: MPCProtocol,
    vm: StoffelVM,
=======
use vm::StoffelVM;
use crate::mpc::protocol;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Player<V: StoffelVM, M: MPCProtocol> {
    id: str,
    vm: V<M>,
    socket: SocketAddr, 
>>>>>>> db8ffc1d3d47abc265f41aec28c6c3204b2d87fb
}

impl Player {
    fn new() -> Self {
        todo!();
    }

<<<<<<< HEAD
    fn send_to_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn receive_from_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn broadcast_and_receive() -> Result<(), Box<dyn std::error::Error>> {
=======
    fn send_to_player() -> Result<()> {
        todo!();
    }

    fn receive_from_player() -> Result<()> {
        todo!();
    }

    fn broadcast_and_receive() -> Result<()> {
>>>>>>> db8ffc1d3d47abc265f41aec28c6c3204b2d87fb
        todo!();
    }

}