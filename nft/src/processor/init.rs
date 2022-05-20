use std::cmp::min;

use borsh::BorshSerialize;
use solana_program::{
    account_info::next_account_info,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};
use mpl_token_metadata;

use crate::{ferror, state::*, utils::*};

pub fn process_init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let ass_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;

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
            9,
        )?,
        &[signer_info.clone(), mint_info.clone(), rent_info.clone(), token_program_info.clone(), ],
    )?;

    msg!("Create Associated Token Account");
    invoke(
        &create_associated_token_account(
            signer_info.key,
            signer_info.key,
            mint_info.key,
        ),
        &[
            signer_info.clone(),
            mint_info.clone(),
            ass_info.clone()
        ],
    )?;

    msg!("Mint Token");
    let mint_to_infos = vec![
        token_program_info.clone(),
        mint_info.clone(),
        signer_info.clone()
    ];
    invoke(
        &mint_to(
            token_program_info.key,
            mint_info.key,
            ass_info.key,
            signer_info.key,
            &[signer_info.key],
            1,
        )?,
        mint_to_infos.as_slice(),
    )?;

    msg!("Create Metadata Account");
    let account_info = vec![
        metadata_info.clone(),
        token_program_info.clone(),
        system_info.clone(),
        signer_info.clone(),
        rent_info.clone(),
    ];
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: creator_key,
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: auth,
            verified: false,
            share: 0,
        },
    ];
    let symbol = String::from("symbol");
    invoke(
        &create_metadata_accounts_v2(
            token_program_info.key,
            metadata_info.key,
            mint_info.key,
            signer_info.key,
            signer_info.key,
            signer_info.key,
            title,
            symbol,
            uri,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;

    msg!("Create Master Edition");
    let master_edition_infos = vec![
        signer_info.clone(),
        token_program_info.clone(),
        system_info.clone(),
        
    ];
    invoke(
        &create_master_edition_v3(
            token_program_info.key,
            master_edition_infos.key,
            mint_info.key,
            signer_info.key,
            signer_info.key,
            metadata_info.key,
            signer_info.key,
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;

    Ok(())
}
