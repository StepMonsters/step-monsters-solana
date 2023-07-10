use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

pub const SEED_STEP_MONSTER: &str = "step_monster_172336";
pub const SEED_MONSTER: &str = "monster_20230215";
pub const SEED_BATTLE: &str = "battle";
pub const SEED_GAME_CONFIG: &str = "game_config_1701";
pub const SEED_MONSTER_FEATURE_CONFIG: &str = "monster_feature_config_07271508";
pub const SEED_BATTLE_HISTORY: &str = "battle_history_12291655";
pub const SEED_BATTLE_HISTORY_BODIES: &str = "battle_history_bodies";
pub const SEED_TOKEN_ADMIN: &str = "token_admin_12152048";
pub const SEED_REFERRAL_INFO: &str = "referral_202307101818";
pub const MAX_BATTLE_LENGTH: usize = 1;
pub const NUM_MONSTER_VALUE: usize = 6;
pub const NUM_MONSTER_ATTR: usize = 6;
pub const NUM_MONSTER_RACE: usize = 10;
pub const MAX_MONSTER_LENGTH: usize = 1 * NUM_MONSTER_VALUE + 4 * NUM_MONSTER_ATTR + (4 + 8) + 8 + (4 + 1 * 10) + (32 * 2) + 1 + 1 + 8;
pub const MAX_GAME_CONFIG_LENGTH: usize = (4 + (4 + 4 * 6) * 10) * 2;
pub const MAX_MONSTER_FEATURE_CONFIG_LENGTH: usize = (4 + (4 + 2 * 7) * 64) * 4;
pub const MAX_BATTLE_HISTORY_LENGTH: usize = 1 + 8 * 2 + (1 + 14 + 4 * 5) * 2 + (4 + 4 * 40) + (4 + (4 + 3 + 10) * 50);
pub const MAX_BATTLE_HISTORY_BODIES_LENGTH: usize = 4 + (4 + 3 + 10) * 100;
pub const MAX_REFERRAL_INFO_LENGTH: usize = 32 * 2 + 8 * 2 + (4 + 8);

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

    pub father_mint: Pubkey,
    pub mother_mint: Pubkey,
    pub walk_target: u8,
    pub rarity: u8,
    pub extra: u64,
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
    pub current_id: u32,
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
    pub const LEN: usize = 1 + 32 + 32 + 32 + 4 + 8 + 2 + 32 + 10 + 200;

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
    pub race: u8,
    pub level: u8,
    pub gender: u8,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub agility: u32,
    pub enemy_feature: Vec<u8>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct CureArgs {
    pub cure: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct TransferSpendingArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct SpendingAccount {
    pub amount: u64,
}

impl SpendingAccount {
    pub const LEN: usize = 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<SpendingAccount, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct MintArgs {
    pub race: u8,
    pub attrs: Vec<u8>,
    pub generation: u8,
    pub father_mint: Pubkey,
    pub mother_mint: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct QuickMintArgs {
    pub race: u8,
    pub attrs: Vec<u8>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BattleHistory {
    pub win: u8,
    pub token: u64,
    pub soul: u64,

    pub me_race: u8,
    pub me_feature: Vec<u8>,
    pub me_hp: u32,
    pub me_attack: u32,
    pub me_defense: u32,
    pub me_speed: u32,
    pub me_agility: u32,

    pub enemy_race: u8,
    pub enemy_feature: Vec<u8>,
    pub enemy_hp: u32,
    pub enemy_attack: u32,
    pub enemy_defense: u32,
    pub enemy_speed: u32,
    pub enemy_agility: u32,

    pub history: Vec<u32>,
    pub bodies: Vec<Vec<u8>>,
}

impl BattleHistory {
    pub fn from_account_info(a: &AccountInfo) -> Result<BattleHistory, ProgramError> {
        if a.data_len() != MAX_BATTLE_HISTORY_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BattleHistoryBodies {
    pub bodies: Vec<Vec<u8>>,
}

impl BattleHistoryBodies {
    pub fn from_account_info(a: &AccountInfo) -> Result<BattleHistoryBodies, ProgramError> {
        if a.data_len() != MAX_BATTLE_HISTORY_BODIES_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ReferralInfo {
    pub addr: Pubkey,
    pub father_addr: Pubkey,
    pub invited: u64,
    pub reward: u64,
    pub ref_code: String,
}

impl ReferralInfo {
    pub fn from_account_info(a: &AccountInfo) -> Result<ReferralInfo, ProgramError> {
        if a.data_len() != MAX_REFERRAL_INFO_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct CreateReferralInfoArgs {
    pub ref_code: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct RecycleArgs {
    pub index: u8,
    pub race: u8,
    pub level: u8,
    pub gender: u8,
    pub soul: u64,
    pub hp: u64,
    pub attack: u64,
    pub defense: u64,
    pub speed: u64,
    pub agility: u64,
    pub efficiency: u64,
    pub enemy_feature: Vec<u8>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ReviveArgs {
    pub index: u8,
    pub race: u8,
    pub level: u8,
    pub gender: u8,
    pub soul: u64,
    pub hp: u64,
    pub attack: u64,
    pub defense: u64,
    pub speed: u64,
    pub agility: u64,
    pub efficiency: u64,
    pub enemy_feature: Vec<u8>,
}

pub fn now_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}
