//! Error types
use {
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that may be returned by the chating program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ChatingError {
    // comm error
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("IncorrectProgramId")]
    IncorrectProgramId,
    #[error("NotRentExempt")]
    NotRentExempt,
    #[error("InvalidSigner")]
    InvalidSigner,
    #[error("InvalidAuthority")]
    InvalidAuthority,
    #[error("InvalidOwner")]
    InvalidOwner,
    // bussiness
    #[error("PostExist")]
    PostExist,
    #[error("PostNotExist")]
    PostNotExist,
    #[error("InvalidSharding")]
    InvalidSharding,
}

impl From<ChatingError> for ProgramError {
    fn from(e: ChatingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ChatingError {
    fn type_of() -> &'static str {
        "Chating Error"
    }
}

impl PrintProgramError for ChatingError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            // comm error
            ChatingError::InvalidInstruction => msg!("Error: invalid instruction"),
            ChatingError::IncorrectProgramId => {
                msg!("Error: the account is not own by this program")
            }
            ChatingError::NotRentExempt => msg!("Error: the account has not enough rent"),
            ChatingError::InvalidSigner => msg!("Error: must sign"),
            ChatingError::InvalidAuthority => msg!("Error: the account has no authority"),
            ChatingError::InvalidOwner => msg!("Error: owner not match"),
            // bussiness
            ChatingError::PostExist => msg!("Error: post is exist, can not overwirte"),
            ChatingError::PostNotExist => msg!("Error: post is not exist"),
            ChatingError::InvalidSharding => {
                msg!("Error: sharding num not match",)
            }
        }
    }
}
