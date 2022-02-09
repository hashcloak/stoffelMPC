use types::numbers::Number;

pub mod honey_badger;

pub trait MPCProtocol: std::fmt::Debug {
    type Public: Number;
    type Secret: Number;

    fn compute();

    fn setup();
}
