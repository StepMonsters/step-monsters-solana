use borsh::BorshSerialize;
use mpl_token_metadata::instruction::{create_master_edition, create_master_edition_v3, create_metadata_accounts_v2};
use mpl_token_metadata::state::Edition;
use mpl_token_metadata::state::Key::EditionV1;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};

use crate::{ferror, state::*, utils::*};

pub fn process_set_white_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let ata_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let white_list_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("Create White List Info");
    let bump_seed = assert_derivation(
        program_id,
        white_list_info,
        &[
            SEED_WHITELIST.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let white_list_seeds = &[
        SEED_WHITELIST.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        white_list_info,
        rent_info,
        system_info,
        signer_info,
        MAX_WHITE_LIST_LENGTH,
        white_list_seeds,
    )?;

    let mut whitelist = WhiteList::from_account_info(white_list_info)?;
    whitelist.name = 1;
    whitelist.serialize(&mut *white_list_info.try_borrow_mut_data()?)?;

    Ok(())
}
