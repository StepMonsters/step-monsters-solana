use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub use init::*;
pub use mint::*;
pub use mint_nft::*;
pub use mint_nft_create::*;
pub use mint_nft_create_data::*;
pub use upgrade::*;
pub use merge::*;
pub use burn::*;
pub use burn_merge::*;
pub use set_white_list::*;

use crate::instruction::*;

pub mod init;
pub mod mint;
pub mod mint_nft;
pub mod mint_nft_create;
pub mod mint_nft_create_data;
pub mod upgrade;
pub mod merge;
pub mod burn;
pub mod burn_merge;
pub mod set_white_list;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::try_from_slice(input)?;
    match instruction {
        GameInstruction::Init() => {
            process_init(program_id, accounts)
        }
        GameInstruction::Mint() => {
            process_mint(program_id, accounts)
        }
        GameInstruction::MintNft() => {
            process_mint_nft(program_id, accounts)
        }
        GameInstruction::CreateNft() => {
            process_mint_nft_create(program_id, accounts)
        }
        GameInstruction::CreateNftData() => {
            process_mint_nft_create_data(program_id, accounts)
        }
        GameInstruction::Upgrade() => {
            process_upgrade(program_id, accounts)
        }
        GameInstruction::Merge() => {
            process_merge(program_id, accounts)
        }
        GameInstruction::Burn() => {
            process_burn(program_id, accounts)
        }
        GameInstruction::BurnMerge() => {
            process_burn_merge(program_id, accounts)
        }
        GameInstruction::SetWhiteList() => {
            process_set_white_list(program_id, accounts)
        }
    }
}
