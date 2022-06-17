use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    pubkey::Pubkey,
};
use spl_token::instruction::{burn};

use crate::{state::*};

pub fn process_synthesis(
    _: &Pubkey,
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
    monster.hp += 200;
    monster.serialize(&mut *monster_info_01.try_borrow_mut_data()?)?;

    Ok(())
}
