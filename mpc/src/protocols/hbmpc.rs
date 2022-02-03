use super::MPCProtocol;
use types::numbers::int::SecInt;
use types::numbers::Number;

pub struct HoneyBadgerMPC<T, U>;

impl<T: Number> MPCProtocol<SecInt<T>> for HoneyBadgerMPC {
    fn compute(number: SecInt<T>) {
        todo!()
    }

    fn setup() {
        todo!()
    }
}
