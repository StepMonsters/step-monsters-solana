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
    Battle,
}


pub fn battle(
    program_id: &Pubkey,
    admin: &Pubkey,
    config: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    pda_create: &Pubkey,
    fee_recevier: &Pubkey,
    metadata: &Pubkey,
    edition: &Pubkey,
    nft_program_id: &Pubkey,
    metadata_program: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new(*pda_create, false),
        AccountMeta::new(*fee_recevier, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new(*metadata, false),
        AccountMeta::new(*edition, false),
        AccountMeta::new_readonly(*nft_program_id, false),
        AccountMeta::new_readonly(*metadata_program, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::Battle.try_to_vec().unwrap(),
    })
}