pub mod ark;
pub mod mpc;

pub use ark::*;
pub use mpc::*;

/// Register address type for indicating which register to use
pub type RegisterAddr = usize;

impl MpcType for RegisterAddr {}

/// Memory address type for indicating memory indexes.
pub type MemoryAddr = usize;

/// Immediate value type for clear values
/// TODO: Change this to be of a specific Number type instead of u64
/// TODO: I think that this is not needed because an inmediate value will be an MpcType.
pub type ImmediateValue = u64;
