use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};

use crate::{utils::*};

pub fn process_mint_init(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let ata_info = next_account_info(account_info_iter)?;

    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("Create Account");
    invoke(
        &system_instruction::create_account(
            signer_info.key,
            mint_info.key,
            required_lamports,
            size as u64,
            token_program_info.key,
        ),
        &[signer_info.clone(), mint_info.clone()],
    )?;

    msg!("Initialize Mint");
    invoke(
        &initialize_mint(
            token_program_info.key,
            mint_info.key,
            signer_info.key,
            Some(signer_info.key),
            0,
        )?,
        &[signer_info.clone(), mint_info.clone(), rent_info.clone(), token_program_info.clone()],
    )?;

    msg!("Create Associated Token Account");
    invoke(
        &create_associated_token_account(
            signer_info.key,
            signer_info.key,
            mint_info.key,
            token_program_info.key
        ),
        &[
            signer_info.clone(),
            ata_info.clone(),
            ass_token_program_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    msg!("Mint To");
    invoke(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            ata_info.key,
            signer_info.key,
            &[signer_info.key],
            1,
        )?,
        &[
            signer_info.clone(),
            ata_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    Ok(())
}
