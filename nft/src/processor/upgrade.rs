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

pub fn process_upgrade(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    msg!("Upgrade Monster");
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.level += 1;
    monster.gender += 1;
    monster.race += 1;
    monster.breed += 1;

    monster.hp += 100;
    monster.attack += 100;
    monster.defense += 100;
    monster.agility += 100;
    monster.luck += 100;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
