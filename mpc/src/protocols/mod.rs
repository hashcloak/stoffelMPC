use types::numbers::Number;

pub mod hbmpc;

pub trait MPCProtocol<T: Number>: std::fmt::Debug {
    type Public<U: Number>: Number;
    fn compute();

    fn setup();
}
