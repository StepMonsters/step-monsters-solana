use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

pub const SEED_MONSTER: &str = "monster";
pub const SEED_BATTLE: &str = "battle";
pub const SEED_GAME_CONFIG: &str = "game_config_1701";
pub const SEED_MONSTER_FEATURE_CONFIG: &str = "monster_feature_config_06301313";
pub const MAX_BATTLE_LENGTH: usize = 1;
pub const NUM_MONSTER_VALUE: usize = 6;
pub const NUM_MONSTER_ATTR: usize = 6;
pub const NUM_MONSTER_RACE: usize = 10;
pub const MAX_MONSTER_LENGTH: usize = 1 * NUM_MONSTER_VALUE + 4 * NUM_MONSTER_ATTR + (4 + 8) + 8 + (4 + 1 * 10);
pub const MAX_GAME_CONFIG_LENGTH: usize = (4 + (4 + 4 * 6) * 10) * 2;
pub const MAX_MONSTER_FEATURE_CONFIG_LENGTH: usize = (4 + (4 + 2 * 7) * 64) * 9;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    Monster,
    Battle,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct Monster {
    pub level: u8,
    pub gender: u8,
    pub race: u8,
    pub breed: u8,
    pub generation: u8,
    pub fatigue: u8,

    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub agility: u32,
    pub efficiency: u32,

    pub energy: u32,
    pub last_battle_time: u64,

    pub hatch_time: u64,
    pub monster_feature: Vec<u8>,
}

impl Monster {
    pub fn from_account_info(a: &AccountInfo) -> Result<Monster, ProgramError> {
        if a.data_len() != MAX_MONSTER_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn calculate_energy(&self) -> u32 {
        let max_count: u64 = (self.efficiency * 2 * 10000 / 24) as u64;
        let hour_per_energy = 24 * 10000 * 10000 / max_count;
        let second_per_energy = hour_per_energy * 60 * 60;
        let mut time_past = now_timestamp() - self.hatch_time;
        if self.last_battle_time > 0 {
            time_past = now_timestamp() - self.last_battle_time;
        }
        let energy_recover = time_past * 10000 * 10000 / second_per_energy;
        let mut now_energy = self.energy + energy_recover as u32;
        if now_energy > 60000 {
            now_energy = 60000;
        }
        return now_energy;
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Battle {
    pub winner: u8,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct ClaimMonsterArgs {
    pub uri: String,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameConfig {
    pub monster_male: Vec<Vec<u32>>,
    pub monster_female: Vec<Vec<u32>>,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MonsterFeatureConfig {
    pub monster_0: Vec<Vec<u16>>,
    pub monster_1: Vec<Vec<u16>>,
    pub monster_2: Vec<Vec<u16>>,

    pub monster_3: Vec<Vec<u16>>,
    pub monster_4: Vec<Vec<u16>>,
    pub monster_5: Vec<Vec<u16>>,

    pub monster_6: Vec<Vec<u16>>,
    pub monster_7: Vec<Vec<u16>>,
    pub monster_8: Vec<Vec<u16>>,
}

impl Battle {
    pub fn from_account_info(a: &AccountInfo) -> Result<Battle, ProgramError> {
        if a.data_len() != MAX_BATTLE_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let battle: Battle = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(battle)
    }
}

impl GameConfig {
    pub fn from_account_info(a: &AccountInfo) -> Result<GameConfig, ProgramError> {
        if a.data_len() != MAX_GAME_CONFIG_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

impl MonsterFeatureConfig {
    pub fn from_account_info(a: &AccountInfo) -> Result<MonsterFeatureConfig, ProgramError> {
        if a.data_len() != MAX_MONSTER_FEATURE_CONFIG_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
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
    /// fee_receiver
    pub fee_receiver: Pubkey,
    /// current id
    pub current_id: u16,
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
    pub const LEN: usize = 1 + 32 + 32 + 32 + 2 + 8 + 4 + 32 + 10 + 200;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        return self.is_initialized;
    }
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct Incubator {
    pub timestamp: u64,
    pub user: Pubkey,
    pub nft: Pubkey,
    pub nft_return: Pubkey,
    pub nft_store: Pubkey,
    pub is_done: bool,
}

impl Incubator {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<Incubator, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BattleArgs {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct CureArgs {
    pub cure: u8,
}

pub fn now_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
