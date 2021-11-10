use super::secret_sharing::SecretSharing;

/// Public fixed point type
///
/// This type is used for providing arithmetic for fixed point numbers
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PubFixed();

/// Secret fixed point type
///
/// This type wraps different implementations for secret fixed point types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecFixed<T: SecretSharing>(T);
