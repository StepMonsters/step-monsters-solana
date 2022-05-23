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

pub fn process_mint_nft_create_data(
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

    let metadata_program_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("Create Monster Info");
    let bump_seed = assert_derivation(
        program_id,
        monster_info,
        &[
            SEED_MONSTER.as_bytes(),
            program_id.as_ref(),
            &mint_info.key.as_ref(),
        ],
    )?;
    let monster_seeds = &[
        SEED_MONSTER.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        monster_info,
        rent_info,
        system_info,
        signer_info,
        MAX_MONSTER_LENGTH,
        monster_seeds,
    )?;

    let mut monster = Monster::from_account_info(monster_info)?;
    monster.level = 1;
    monster.gender = 1;
    monster.race = 1;
    monster.breed = 1;

    monster.hp = 100;
    monster.attack = 100;
    monster.defense = 100;
    monster.agility = 100;
    monster.luck = 100;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
