use borsh::BorshSerialize;
use mpl_token_metadata::instruction::create_master_edition;
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
use crate::instruction::mint;

pub fn process_burn_merge(
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
    let monster_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    let size = 82;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("Burn");
    invoke(
        &burn(
            token_program_info.key,
            ata_info.key,
            mint_info.key,
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

    msg!("Upgrade Monster");
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.level += 2;
    monster.gender += 2;
    monster.race += 2;
    monster.breed += 2;

    monster.hp += 200;
    monster.attack += 200;
    monster.defense += 200;
    monster.agility += 200;
    monster.luck += 200;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;
    
    Ok(())
}
