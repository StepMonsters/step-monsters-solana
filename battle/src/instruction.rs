use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GameInstruction {
    Battle(BattleArgs),
}


pub fn create_battle(
    program_id: &Pubkey,
    admin: &Pubkey,
    authority: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    nft_program_id: &Pubkey,
    token_program: &Pubkey,
    args: BattleArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*authority, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*admin, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new_readonly(*nft_program_id, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::Battle(args).try_to_vec().unwrap(),
    })
}