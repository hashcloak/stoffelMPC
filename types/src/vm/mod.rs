pub mod ark;
pub mod mpc;

pub use ark::*;
pub use mpc::*;

/// Register address type for indicating which register to use
pub type RegisterAddr = usize;

/// Immediate value type for clear values
/// TODO: Change this to be of a specific Number type instead of u64
pub type ImmediateValue = u64;
