use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod init;
pub use init::*;


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StakePoolInstruction::try_from_slice(input)?;
    match instruction {
        StakePoolInstruction::Init() => {
            msg!("Instruction: Init");
            process_init(program_id, accounts)
        }
    }
}
