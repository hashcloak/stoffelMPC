use types::numbers::Number;

pub mod hbmpc;

pub trait MPCProtocol<T: Number> {
    type Public<U: Number>: Number;
    fn compute();

    fn setup();
}
