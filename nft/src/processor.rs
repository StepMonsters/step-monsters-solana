use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub use battle::*;
pub use breed::*;
pub use claim_monster::*;
pub use config_game::*;
pub use config_monster_feature::*;
pub use configure::*;
pub use cure::*;
pub use hatch::*;
pub use hatch_quick::*;
pub use mint::*;
pub use mint_init::*;
pub use synthesis::*;
pub use upgrade::*;

use crate::instruction::*;

pub mod configure;
pub mod mint_init;
pub mod mint;
pub mod hatch_quick;
pub mod hatch;
pub mod claim_monster;
pub mod battle;
pub mod breed;
pub mod synthesis;
pub mod upgrade;
pub mod config_game;
pub mod config_monster_feature;
pub mod cure;

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
        GameInstruction::InitMint => {
            process_mint_init(program_id, accounts)
        }
        GameInstruction::Mint => {
            process_mint(program_id, accounts)
        }
        GameInstruction::QuickHatch => {
            process_hatch_quick(program_id, accounts)
        }
        GameInstruction::Hatch => {
            process_hatch(program_id, accounts)
        }
        GameInstruction::ClaimMonster(args) => {
            process_claim_monster(program_id, accounts, args)
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
        GameInstruction::Cure(args) => {
            process_cure(program_id, accounts, args)
        }
    }
}
