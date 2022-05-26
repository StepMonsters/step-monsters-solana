use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction::burn;

use crate::{state::*, utils::*};

pub fn process_burn_merge(
    _: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let ata_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

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
    monster.hp += 200;
    monster.attack += 200;
    monster.defense += 200;
    monster.agility += 200;
    monster.luck += 200;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
