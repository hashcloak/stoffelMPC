use super::secret_sharing::SecretSharing;

/// Public Gf2 type
///
/// This type is used for providing finite field arithmetic over two elements
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubGf2<const N: usize>([bool; N]);

/// Secret Gf2 type
///
/// This type wraps different implementation for secret Gf2 types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecGf2<T: SecretSharing>(T);
