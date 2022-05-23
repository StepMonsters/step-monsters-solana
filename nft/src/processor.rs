use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub use init::*;
pub use mint::*;
pub use mint_nft::*;
pub use mint_nft_create::*;
pub use mint_nft_create_data::*;

use crate::instruction::*;

pub mod init;
pub mod mint;
pub mod mint_nft;
pub mod mint_nft_create;
pub mod mint_nft_create_data;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StakePoolInstruction::try_from_slice(input)?;
    match instruction {
        StakePoolInstruction::Init() => {
            process_init(program_id, accounts)
        }
        StakePoolInstruction::Mint() => {
            process_mint(program_id, accounts)
        }
        StakePoolInstruction::MintNft() => {
            process_mint_nft(program_id, accounts)
        }
        StakePoolInstruction::CreateNft() => {
            process_mint_nft_create(program_id, accounts)
        }
        StakePoolInstruction::CreateNftData() => {
            process_mint_nft_create_data(program_id, accounts)
        }
    }
}
