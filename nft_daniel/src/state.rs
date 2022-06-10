use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const SEED_MONSTER: &str = "monster";
pub const SEED_BATTLE: &str = "battle";
pub const MAX_MONSTER_LENGTH: usize = 1 * 4 + 4 * 5;
pub const MAX_BATTLE_LENGTH: usize = 1;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    Monster,
    Battle,
}

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
pub struct Battle {
    pub winner: u8,
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

impl Battle {
    pub fn from_account_info(a: &AccountInfo) -> Result<Battle, ProgramError> {
        if a.data_len() != MAX_BATTLE_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let battle: Battle =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(battle)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureArgs {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// creator
    pub creator: Pubkey,
    /// fee_recevier
    pub fee_recevier: Pubkey,
    /// nft price
    pub price: u64,
    /// seller fee
    pub fee: u16,
    /// nft name
    pub name: String,
    /// nft symbol
    pub symbol: String,
    /// default uri
    pub uri: String,
}

pub type ConfigureData = ConfigureArgs;

impl ConfigureData {
    pub const LEN: usize = 1 + 32 + 32 + 32 + 8 + 4 + 32 + 10 + 200;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        return self.is_initialized
    
    }
}


