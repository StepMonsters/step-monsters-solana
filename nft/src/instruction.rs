use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GameInstruction {
    Init(),
    Mint(),
    MintNft(),
    CreateNft(),
    CreateNftData(),

    Upgrade(),
    Merge(),
    Burn(),
    BurnMerge(),
    SetWhiteList(),

    Purchase(),
    CreateArray(),
    UpdateArray(),
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
        data: GameInstruction::Init().try_to_vec().unwrap(),
    })
}
