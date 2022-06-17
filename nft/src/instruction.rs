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
    Configure(ConfigureArgs),
    Mint,
    Hatch,
    Breed,
    Synthesis,
    Upgrade,
    Battle(BattleArgs),
    CreateGameConfig(),
    UpdateGameConfig(),
    CreateMonsterFeatureConfig(),
}


pub fn config(
    program_id: &Pubkey,
    signer: &Pubkey,
    config: &Pubkey,
    args: ConfigureArgs
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn mint(
    program_id: &Pubkey,
    signer: &Pubkey,
    config: &Pubkey,
    pda_creator: &Pubkey,
    creator: &Pubkey,
    mint: &Pubkey,
    metadata: &Pubkey,
    edition: &Pubkey,
    metadata_program: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new(*pda_creator, false),
        AccountMeta::new(*creator, false),
        AccountMeta::new(*mint, true),
        AccountMeta::new(*metadata, false),
        AccountMeta::new(*edition, false),
        AccountMeta::new_readonly(*metadata_program, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::Mint.try_to_vec().unwrap(),
    })
}

pub fn battle(
    program_id: &Pubkey,
    admin: &Pubkey,
    config: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    pda_create: &Pubkey,
    metadata: &Pubkey,
    edition: &Pubkey,
    metadata_program: &Pubkey,
    token_program: &Pubkey,
    args: BattleArgs
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new(*pda_create, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new(*metadata, false),
        AccountMeta::new(*edition, false),
        AccountMeta::new_readonly(*metadata_program, false),
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