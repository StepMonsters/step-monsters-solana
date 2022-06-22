use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod configure;
pub use configure::*;

pub mod mint;
pub use mint::*;

pub mod hatch;
pub use hatch::*;

pub mod claim_monster;
pub use claim_monster::*;

pub mod battle;
pub use battle::*;

pub mod breed;
pub use breed::*;

pub mod synthesis;
pub use synthesis::*;

pub mod upgrade;
pub use upgrade::*;

pub mod game_config;
pub use game_config::*;

pub mod config_monster_feature;
pub use config_monster_feature::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::try_from_slice(input)?;
    match instruction {
        GameInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        GameInstruction::Mint => {
            process_mint(program_id, accounts)
        }
        GameInstruction::Hatch => {
            process_hatch(program_id, accounts)
        }
        GameInstruction::ClaimMonster => {
            process_claim_monster(program_id, accounts)
        }
        GameInstruction::Breed => {
            process_breed(program_id, accounts)
        }
        GameInstruction::Upgrade => {
            process_upgrade(program_id, accounts)
        }
        GameInstruction::Synthesis => {
            process_synthesis(program_id, accounts)
        }
        GameInstruction::Battle(args) => {
            process_battle(program_id, accounts, args)
        }
        GameInstruction::CreateGameConfig() => {
            process_create_game_config(program_id, accounts)
        }
        GameInstruction::UpdateGameConfig() => {
            process_update_game_config(program_id, accounts)
        }
        GameInstruction::CreateMonsterFeatureConfig() => {
            process_create_monster_feature_config(program_id, accounts)
        }
    }
}
