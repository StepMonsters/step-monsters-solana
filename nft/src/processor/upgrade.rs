use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};

pub fn process_upgrade(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Upgrade Monster");
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.level += 1;
    monster.hp = monster.hp * 106 / 100;
    monster.attack = monster.attack * 106 / 100;
    monster.defense = monster.defense * 106 / 100;
    monster.speed = monster.speed * 106 / 100;
    monster.agility = monster.agility * 106 / 100;
    monster.efficiency = monster.efficiency * 106 / 100;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
