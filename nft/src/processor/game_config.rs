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

pub fn process_create_array(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create Game Config");
    let bump_seed = assert_derivation(
        program_id,
        game_config_info,
        &[
            SEED_GAME_CONFIG.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let game_config_seeds = &[
        SEED_GAME_CONFIG.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        game_config_info,
        rent_info,
        system_info,
        signer_info,
        MAX_GAME_CONFIG_LENGTH,
        game_config_seeds,
    )?;

    let array = [1, 2, 3, 4, 5];
    let mut game_config = GameConfig::from_account_info(game_config_info)?;
    game_config.array = array;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}

pub fn process_update_array(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_config_info = next_account_info(account_info_iter)?;

    let mut game_config = GameConfig::from_account_info(game_config_info)?;
    let mut array = game_config.array.clone();

    for i in 0..array.len() {
        array[i] += 1;
    }

    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}