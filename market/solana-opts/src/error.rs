use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum OptError {
    #[error("Invalid signer")]
    InvalidSigner = 0xca01,

    #[error("Invalid derived key")]
    InvalidDerivedKey = 0xca02,

    #[error("Not Rent Exempt")]
    NotRentExempt = 0xca03,

    #[error("Invalid owner")]
    InvalidOwner = 0xca04,

    #[error("Already initialized")]
    AlreadyInitialized = 0xca05,

    #[error("Uninitialized")]
    Uninitialized = 0xca06,

    #[error("Invalid associated address")]
    InvalidAssociatedAddress = 0xca07,

    #[error("Invalid eq pubkey")]
    InvalidEqPubkey = 0xca08,

    #[error("Token transfer failed")]
    TokenTransferFailed = 0xca09,

    #[error("Checked calculate failed")]
    CheckedCalculateFailed = 0xca0a,

    #[error("Invalid data empty")]
    InvalidDataEmpty = 0xca0b,

    #[error("Invalid data not empty")]
    InvalidDataNotEmpty = 0xca0c,

    #[error("Insufficient Funds")]
    InsufficientFundsForTransaction = 0xca0d,
}

impl From<OptError> for ProgramError {
    fn from(err: OptError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
