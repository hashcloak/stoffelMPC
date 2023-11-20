use mpc::protocols::{honey_badger::HoneyBadgerMPC, MPCProtocol};
use tokio::net::TcpListener;
use vm::processor::arithmetic::ArithmeticCore;
use vm::processor::Processor;
use vm::Program;
use vm::StoffelVM;

pub struct Player<T: MPCProtocol, U: Processor> {
    id: String,
    vm: StoffelVM<U, T>,
    recipients: Vec<std::io::BufWriter<Box<dyn std::io::Write>>>,
    reader: std::io::BufReader<Box<dyn std::io::Read>>,
    protocol: std::marker::PhantomData<T>,
}

impl<T: MPCProtocol, U: Processor> Player<T, U> {
    fn send_to_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn receive_from_player() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    fn broadcast_and_receive() -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }

    pub async fn run(
        program: Program<U>,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }
}

// example to instantiate a specific Player
impl Player<HoneyBadgerMPC, ArithmeticCore<HoneyBadgerMPC>> {
    pub fn new() -> Player<HoneyBadgerMPC, ArithmeticCore<HoneyBadgerMPC>> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
