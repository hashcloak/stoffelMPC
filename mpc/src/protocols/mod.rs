use crate::types::{PubInt, SecInt};
use std::io::{Read, Write};

mod honey_badger;

pub trait Channel: Read + Write {}

pub trait Protocol<T>: Open<T> + Channel {
    // This definitely not final
    fn preprocess();
    fn run();
}

pub trait Open<T> {
    fn open<U: Channel>(channel: &mut U, secret: SecInt<T>) -> PubInt;
}
