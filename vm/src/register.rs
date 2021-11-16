use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
};

pub trait Register {}

impl Register for PubInt {}

impl<T: SecretSharing> Register for SecInt<T> {}

impl Register for PubFixed {}

impl<T: SecretSharing> Register for SecFixed<T> {}

impl<const N: usize> Register for PubGf2<N> {}

impl<T: SecretSharing> Register for SecGf2<T> {}
