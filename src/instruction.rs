//! Instruction types

use crate::error::ChatingError;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use arrayref::{array_ref, array_refs};

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]

/// Stake program contains the following instructions
/// * `Inintialize`
/// * `UpdateReistrar`
/// * `CreateMember`
/// * `UpdateMember`
/// * `Stake`
/// * `Unstake`
/// * `ExpireReward`
/// * `Terminate`
pub enum ChatingInstruction {
    //instruction
    CreatePost {
        content: String,
    },
    CreatePostSharding {
        sharding: u64,
        master_key: Pubkey,
        content: String,
    },
    Terminate {},
}

// unpack instruction , pack instruction
impl ChatingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(crate::error::ChatingError::InvalidInstruction)?;
        Ok(match tag {
            // post
            0 => {
                let content = std::str::from_utf8(rest).unwrap();
                Self::CreatePost {
                    content: String::from(content),
                }
            }
            // post sharding
            1 => {
                let data = array_ref![rest, 0, 8 + 32 + CONTENT_LENGTH];
                let (sharding_buf, master_key_buf, content_buf) =
                    array_refs![data, 8, 32, CONTENT_LENGTH];
                let content = std::str::from_utf8(content_buf).unwrap();
                Self::CreatePostSharding {
                    sharding: u64::from_le_bytes(*sharding_buf),
                    master_key: Pubkey::new_from_array(*master_key_buf),
                    content: String::from(content),
                }
            }

            10 => Self::Terminate {},

            _ => return Err(ChatingError::InvalidInstruction.into()),
        })
    }
}

const CONTENT_LENGTH: usize = 512;
