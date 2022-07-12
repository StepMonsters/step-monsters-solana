use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_cure(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Cure Monster Fatigue");
    let mut monster = Monster::from_account_info(monster_info)?;
    if (args.cure == 1 || args.cure == 25 || args.cure == 50 || args.cure == 75 || args.cure == 100) && monster.fatigue >= args.cure {
        monster.fatigue -= args.cure;
    } else {
        return ferror!("Invalid cure.");
    }
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
