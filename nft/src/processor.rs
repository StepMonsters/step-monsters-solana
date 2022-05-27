use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub use breed::*;
pub use game_config::*;
pub use mint::*;
pub use synthesis::*;
pub use upgrade::*;

use crate::instruction::*;

pub mod mint;
pub mod upgrade;
pub mod synthesis;
pub mod game_config;
pub mod breed;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::try_from_slice(input)?;
    match instruction {
        GameInstruction::Mint() => {
            process_mint(program_id, accounts)
        }
        GameInstruction::Breed() => {
            process_breed(program_id, accounts)
        }
        GameInstruction::Upgrade() => {
            process_upgrade(program_id, accounts)
        }
        GameInstruction::Synthesis() => {
            process_synthesis(program_id, accounts)
        }
        GameInstruction::CreateGameConfig() => {
            process_create_game_config(program_id, accounts)
        }
        GameInstruction::UpdateGameConfig() => {
            process_update_game_config(program_id, accounts)
        }
    }
}
