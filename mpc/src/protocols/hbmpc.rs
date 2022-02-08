use super::MPCProtocol;
use types::numbers::gf2::{PubGf2, SecGf2};
use types::numbers::int::{PubInt, SecInt};
use types::numbers::Number;

#[derive(Debug)]
pub struct HoneyBadgerMPC;

impl<T: Number> MPCProtocol<SecInt<T>> for HoneyBadgerMPC {
    type Public<U: Number> = PubInt<U>;
    fn compute() {
        todo!()
    }

    fn setup() {
        todo!()
    }
}

impl<T: Number> MPCProtocol<SecGf2<T>> for HoneyBadgerMPC {
    type Public<U: Number> = PubGf2<U>;

    fn compute() {
        todo!()
    }

    fn setup() {
        todo!()
    }
}
