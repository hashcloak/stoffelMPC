use types::{PubFixed, PubGf2, PubInt, SecFixed, SecGf2, SecInt};

pub trait Register {}

impl Register for PubInt {}

impl<T> Register for SecInt<T> {}

impl Register for PubFixed {}

impl<T> Register for SecFixed<T> {}

impl Register for PubGf2 {}

impl<T> Register for SecGf2<T> {}
