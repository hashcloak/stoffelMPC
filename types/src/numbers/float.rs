use super::secret_sharing::SecretSharing;

/// Public floating point type
///
/// This type is used for providing arithmetic for floating point numbers
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PubFloat(f32);

/// Secret floating point type
///
/// This type wraps different implementations for secret floating point types
/// in order to provide a stable API for every type it wraps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecFloat<T: SecretSharing>(T);
