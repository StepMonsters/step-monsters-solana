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
use spl_token::instruction::{burn, initialize_mint, mint_to};

use crate::{ferror, state::*, utils::*};

pub fn process_merge(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let ata_info_02 = next_account_info(account_info_iter)?;
    let mint_info_02 = next_account_info(account_info_iter)?;
    let monster_info_01 = next_account_info(account_info_iter)?;

    msg!("Burn Mint");
    invoke(
        &burn(
            token_program_info.key,
            ata_info_02.key,
            mint_info_02.key,
            signer_info.key,
            &[signer_info.key],
            1,
        )?,
        &[
            signer_info.clone(),
            ata_info_02.clone(),
            mint_info_02.clone(),
            token_program_info.clone(),
            system_info.clone()
        ],
    )?;

    msg!("Upgrade Monster");
    let mut monster = Monster::from_account_info(monster_info_01)?;
    monster.level += 2;
    monster.gender += 2;
    monster.race += 2;
    monster.breed += 2;

    monster.hp += 200;
    monster.attack += 200;
    monster.defense += 200;
    monster.agility += 200;
    monster.luck += 200;
    monster.serialize(&mut *monster_info_01.try_borrow_mut_data()?)?;

    Ok(())
}
