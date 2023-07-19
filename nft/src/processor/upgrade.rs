use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_config::calculate_upgrade_spend_game_token;

pub fn process_upgrade(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let mut monster = Monster::from_account_info(monster_info)?;

    if monster.level >= 30 {
        return ferror!("Reach max level.");
    };

    msg!("Upgrade LST spending");
    let spend = calculate_upgrade_spend_game_token(monster.race, monster.level);
    spl_token_transfer_invoke(
        token_program_info.clone(),
        signer_ata_info.clone(),
        program_ata_info.clone(),
        signer_info.clone(),
        spend,
    )?;

    msg!("Upgrade Monster");
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
