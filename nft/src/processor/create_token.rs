use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use solana_program::program::invoke_signed;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};

use crate::{ferror, utils::*};
use crate::state::{ConfigureData, SEED_TOKEN_ADMIN};
use crate::utils_mint::create_token_admin_info;

pub fn process_create_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;

    let signer_ata_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;
    let token_admin_info = next_account_info(account_info_iter)?;

    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    //check authority
    let config_info = next_account_info(account_info_iter)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }

    assert_signer(&signer_info)?;
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    let decimal: u64 = 1_000_000_000;
    let amount: u64 = 1 * decimal;

    msg!("Token Admin Seeds");
    let bump_seed = assert_derivation(
        program_id,
        token_admin_info,
        &[
            SEED_TOKEN_ADMIN.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let token_admin_seeds = [
        SEED_TOKEN_ADMIN.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];

    msg!("Create Token Admin Account");
    if token_admin_info.lamports() <= 0 {
        create_token_admin_info(
            &program_id,
            &token_admin_info,
            &rent_info,
            &system_info,
            &signer_info,
        )?;
    }

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
            token_admin_info.key,
            Some(signer_info.key),
            9,
        )?,
        &[signer_info.clone(), mint_info.clone(), rent_info.clone(), token_program_info.clone(), token_admin_info.clone()],
    )?;

    msg!("Create Signer Associated Token Account");
    invoke(
        &create_associated_token_account(
            signer_info.key,
            signer_info.key,
            mint_info.key,
        ),
        &[
            signer_info.clone(),
            signer_ata_info.clone(),
            ass_token_program_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    msg!("Mint To Signer");
    invoke_signed(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            signer_ata_info.key,
            token_admin_info.key,
            &[token_admin_info.key],
            amount,
        )?,
        &[
            signer_info.clone(),
            signer_ata_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            token_admin_info.clone()
        ],
        &[&token_admin_seeds],
    )?;

    msg!("Create Program Associated Token Account");
    invoke(
        &create_associated_token_account(
            signer_info.key,
            token_admin_info.key,
            mint_info.key,
        ),
        &[
            signer_info.clone(),
            program_ata_info.clone(),
            ass_token_program_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            token_admin_info.clone()
        ],
    )?;

    msg!("Mint To Program Token Admin Account");
    invoke_signed(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            program_ata_info.key,
            token_admin_info.key,
            &[token_admin_info.key],
            amount,
        )?,
        &[
            signer_info.clone(),
            program_ata_info.clone(),
            mint_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            token_admin_info.clone()
        ],
        &[&token_admin_seeds],
    )?;

    Ok(())
}
