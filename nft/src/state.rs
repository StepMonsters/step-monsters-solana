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
pub const SEED_GAME_CONFIG: &str = "game_config";
pub const SEED_MONSTER_FEATURE_CONFIG: &str = "monster_feature_config";
pub const MAX_BATTLE_LENGTH: usize = 1;
pub const NUM_MONSTER_VALUE: usize = 6;
pub const NUM_MONSTER_ATTR: usize = 6;
pub const NUM_MONSTER_RACE: usize = 10;
pub const MAX_MONSTER_LENGTH: usize = 1 * NUM_MONSTER_VALUE + 4 * NUM_MONSTER_ATTR + 8 + 8 + 1 * 20;
pub const MAX_GAME_CONFIG_LENGTH: usize =
    4 * NUM_MONSTER_ATTR * NUM_MONSTER_RACE + 4 * NUM_MONSTER_ATTR * NUM_MONSTER_RACE;
pub const MAX_MONSTER_FEATURE_CONFIG_LENGTH: usize = 2 * 7 * 64 * 5;

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
    pub energy: u32,
    pub efficiency: u32,

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
        //assume 2 energy per hour  max 100
        let energy_per_hour = 2 * self.efficiency;
        let now_ts = now_timestamp();
        if self.last_battle_time > 0 {
            let round = (now_ts - self.last_battle_time) / 3600;
            if round as u32 * energy_per_hour + self.energy > 100 {
                return 100
            } else {
                return round as u32 * energy_per_hour + self.energy
            }
        } else {
            let round = (now_ts - self.hatch_time) / 3600;
            if round as u32 * energy_per_hour + self.energy > 100 {
                return 100
            } else {
                return round as u32 * energy_per_hour + self.energy
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Battle {
    pub winner: u8,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct GameConfig {
    pub monster_male: [[u32; NUM_MONSTER_ATTR]; NUM_MONSTER_RACE],
    pub monster_female: [[u32; NUM_MONSTER_ATTR]; NUM_MONSTER_RACE],
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MonsterFeatureConfig {
    pub monster_0: Vec<[u8; 7]>,
    pub monster_1: Vec<[u8; 7]>,
    pub monster_2: Vec<[u8; 7]>,
    pub monster_3: Vec<[u8; 7]>,
    pub monster_4: Vec<[u8; 7]>,
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
        let game_config: GameConfig = try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(game_config)
    }
}

impl MonsterFeatureConfig {
    pub fn from_account_info(a: &AccountInfo) -> Result<MonsterFeatureConfig, ProgramError> {
        if a.data_len() != MAX_MONSTER_FEATURE_CONFIG_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        let monster_feature_config: MonsterFeatureConfig =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
        Ok(monster_feature_config)
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
        return self.is_initialized;
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BattleArgs {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
}

pub fn now_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
