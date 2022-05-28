use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
};

pub const SEED_MONSTER: &str = "monster";
pub const SEED_GAME_CONFIG: &str = "game_config";
pub const MAX_MONSTER_LENGTH: usize = 1 * 5 + 4 * 6;
pub const MAX_GAME_CONFIG_LENGTH: usize = 4 * 5 * 10 + 4 * 5 * 10;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Monster {
    pub level: u8,
    pub gender: u8,
    pub race: u8,
    pub breed: u8,
    pub generation: u8,

    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub agility: u32,
    pub luck: u32,
    pub speed: u32,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameConfig {
    pub monster_male: [[u32; 5]; 10],
    pub monster_female: [[u32; 5]; 10],
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
