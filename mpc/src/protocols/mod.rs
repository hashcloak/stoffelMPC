use types::numbers::Number;

pub mod hbmpc;

pub trait MPCProtocol<T: Number> {
    fn compute(number: T);

    fn setup();
}
