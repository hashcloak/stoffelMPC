use super::secret_sharing::SecretSharing;

/// Public bit type
///
/// This type is used for providing arithmetic for bits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubBit(bool);

/// Secret bit type
///
/// This type wraps different implementations for secret bit types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecBit<T: SecretSharing>(T);
