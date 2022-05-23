use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::error::MetadataError;
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::utils::try_from_slice_checked;

pub const SEED_MONSTER: &str = "monster";
pub const MAX_MONSTER_LENGTH: usize = 1 * 4 + 4 * 5;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    Monster,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct InitArgs {}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Monster {
    pub level: u8,
    pub gender: u8,
    pub race: u8,
    pub breed: u8,

    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub agility: u32,
    pub luck: u32,
}

impl Monster {
    pub fn from_account_info(a: &AccountInfo) -> Result<Monster, ProgramError> {
        let monster: Monster =
            try_from_slice_checked(&a.data.borrow_mut(), Key::Monster, MAX_MONSTER_LENGTH)?;
        Ok(monster)
    }
}

