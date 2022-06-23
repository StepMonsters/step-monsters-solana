use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*};

pub fn process_upgrade(
    _: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let monster_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;

    msg!("Upgrade Monster");
    let mut monster = Monster::from_account_info(monster_info)?;
    let game_config = GameConfig::from_account_info(game_config_info)?;

    let male = game_config.monster_male.clone();
    let female = game_config.monster_female.clone();

    let mut basic: Vec<u32> = male[monster.race as usize].clone();
    if monster.gender != 1 {
        basic = female[monster.race as usize].clone();
    }

    let multi: u32 = (106 as u32).pow(monster.level as u32) / 100;
    monster.hp = basic[0] * multi;
    monster.level += 1;

    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
