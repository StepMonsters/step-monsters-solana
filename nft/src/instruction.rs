use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    program_error::ProgramError,
    system_program,
    sysvar::{rent},
};
use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum StakePoolInstruction {
    Init(),
    Mint(),
    MintNft(),
    CreateNft(),
    CreateNftData(),
}

pub fn init(
    program_id: &Pubkey,
    authority: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*authority, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::Init().try_to_vec().unwrap(),
    })
}

pub fn mint(
    program_id: &Pubkey,
    authority: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*authority, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::Mint().try_to_vec().unwrap(),
    })
}

pub fn mintNft(
    program_id: &Pubkey,
    authority: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*authority, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::MintNft().try_to_vec().unwrap(),
    })
}

pub fn createNft(
    program_id: &Pubkey,
    authority: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*authority, false),
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::CreateNft().try_to_vec().unwrap(),
    })
}