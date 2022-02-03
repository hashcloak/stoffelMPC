use types::numbers::Number;

pub mod hbmpc;

pub trait MPCProtocol<T: Number> {
    type Public<U: Number>;
    fn compute();

    fn setup();
}
