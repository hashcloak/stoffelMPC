use thiserror::Error;

pub mod shamir;

pub trait SecretSharing: Copy + Default {
    fn share(&mut self) -> Result<(), SecretSharingError>;
}

#[derive(Debug, Error)]
pub enum SecretSharingError {
    #[error("SecretSharingError: {0}")]
    ShamirError(#[from] shamir::ShamirError),
}
