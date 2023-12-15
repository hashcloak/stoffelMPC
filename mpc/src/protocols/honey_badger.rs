use super::MPCProtocol;
use ark_bls12_381::Fr;

#[derive(Debug)]
pub struct HoneyBadgerMPC;

impl MPCProtocol for HoneyBadgerMPC {
    type Domain = Fr;

    fn compute() {
        todo!()
    }

    fn setup() {
        todo!()
    }

    fn send(value: &Self::Domain, recipient: Box<dyn std::io::Write>) {
        todo!()
    }
}
