use std::io::{Read, Write};

mod int;

pub trait Open {
    type Public;
    fn open<U: Read + Write>(self, channel: &mut U) -> Self::Public;
}
