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
    InitMint,
    Mint,
    QuickHatch,
    Hatch,
    ClaimMonster(ClaimMonsterArgs),
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
    monster: &Pubkey,
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
        AccountMeta::new(*monster, false),
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


pub fn hatch(
    program_id: &Pubkey,
    signer: &Pubkey,
    monster: &Pubkey,
    game_config: &Pubkey,
    monster_feature_config: &Pubkey,
    nft_mint: &Pubkey,
    incubator_info: &Pubkey,
    nft_account_info: &Pubkey,
    nft_store_info: &Pubkey,
    authority_info: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*monster, false),
        AccountMeta::new(*game_config, false),
        AccountMeta::new(*monster_feature_config, false),
        AccountMeta::new(*incubator_info, false),
        AccountMeta::new(*nft_mint, false),
        AccountMeta::new(*nft_account_info, false),
        AccountMeta::new(*nft_store_info, false),
        AccountMeta::new(*authority_info, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::Hatch.try_to_vec().unwrap(),
    })
}

pub fn claim_monster(
    program_id: &Pubkey,
    signer: &Pubkey,
    monster: &Pubkey,
    metadata: &Pubkey,
    incubator: &Pubkey,
    pda_creator: &Pubkey,
    nft_mint: &Pubkey,
    nft_account_info: &Pubkey,
    nft_store_info: &Pubkey,
    authority_info: &Pubkey,
    token_program: &Pubkey,
    metadata_program: &Pubkey,
    args: ClaimMonsterArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*monster, false),
        AccountMeta::new(*metadata, false),
        AccountMeta::new(*incubator, false),
        AccountMeta::new(*pda_creator, false),
        AccountMeta::new(*nft_mint, false),
        AccountMeta::new(*nft_account_info, false),
        AccountMeta::new(*nft_store_info, false),
        AccountMeta::new(*authority_info, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(*metadata_program, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: GameInstruction::ClaimMonster(args).try_to_vec().unwrap(),
    })
}