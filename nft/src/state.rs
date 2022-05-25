use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::error::MetadataError;
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const SEED_MONSTER: &str = "monster";
pub const SEED_WHITELIST: &str = "whitelist";
pub const SEED_GAME_CONFIG: &str = "game_config";
pub const MAX_MONSTER_LENGTH: usize = 1 * 4 + 4 * 5;
pub const MAX_WHITE_LIST_LENGTH: usize = 1;
pub const MAX_GAME_CONFIG_LENGTH: usize = 1 * 5;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    Monster,
    WhiteList,
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

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct WhiteList {
    pub name: u8,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameConfig {
    pub array: [u8; 5],
}

impl Monster {
    pub fn from_account_info(a: &AccountInfo) -> Result<Monster, ProgramError> {
        if a.data_len() != MAX_MONSTER_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let monster: Monster =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(monster)
    }
}

impl WhiteList {
    pub fn from_account_info(a: &AccountInfo) -> Result<WhiteList, ProgramError> {
        if a.data_len() != MAX_WHITE_LIST_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let whitelist: WhiteList =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(whitelist)
    }
}

impl GameConfig {
    pub fn from_account_info(a: &AccountInfo) -> Result<GameConfig, ProgramError> {
        if a.data_len() != MAX_GAME_CONFIG_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let game_config: GameConfig =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(game_config)
    }
}
