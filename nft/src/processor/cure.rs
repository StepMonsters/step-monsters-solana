use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_config::calculate_cure_spend_game_token;

pub fn process_cure(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    let admin_fund_info = next_account_info(account_info_iter);

    assert_signer(&signer_info)?;

    msg!("Cure Monster Fatigue");
    let mut monster = Monster::from_account_info(monster_info)?;
    if args.cure == 25 || args.cure == 50 || args.cure == 75 || args.cure == 100 {
        if monster.fatigue <= args.cure {
            monster.fatigue = 0;
        } else {
            monster.fatigue -= args.cure;
        }
        let spend = calculate_cure_spend_game_token(monster.level, args.cure);
        spl_token_transfer_invoke(
            token_program_info.clone(),
            signer_ata_info.clone(),
            program_ata_info.clone(),
            signer_info.clone(),
            spend,
        )?;
    } else {
        return ferror!("Invalid cure.");
    }

    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    //send fund
    send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, 0)?;

    Ok(())
}
