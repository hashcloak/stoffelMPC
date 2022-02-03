use super::MPCProtocol;
use types::numbers::int::{PubInt, SecInt};
use types::numbers::Number;

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
