use super::MPCProtocol;
use types::numbers::Number;

#[derive(Debug)]
pub struct HoneyBadgerMPC<T: Number, U: Number> {
    secret: std::marker::PhantomData<T>,
    public: std::marker::PhantomData<U>,
}

impl<T: Number, U: Number> MPCProtocol for HoneyBadgerMPC<T, U> {
    type Public = U;
    type Secret = T;

    fn compute() {
        todo!()
    }

    fn setup() {
        todo!()
    }
}
