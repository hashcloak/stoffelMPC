use std::io::{Read, Write};

mod honey_badger;

pub trait Protocol: Open {
    // This definitely not final
    fn preprocess();
    fn run();
}

pub trait Open {
    type Public;
    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public;
}
