use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::{create_battle_history_info, spl_token_burn_quick};

pub fn process_recycle(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: RecycleArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_account_info = next_account_info(account_info_iter)?;
    let battle_history_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let admin_fund_info = next_account_info(account_info_iter);
    let _program_info = next_account_info(account_info_iter);

    assert_signer(&signer_info)?;

    if battle_history_info.lamports() <= 0 {
        send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, MAX_BATTLE_HISTORY_LENGTH)?;
        create_battle_history_info(
            &program_id,
            &battle_history_info,
            &rent_info,
            &system_info,
            &signer_info,
            false,
        )?;
    }

    msg!("Recycle Monster");
    let mut battle_history = BattleHistory::from_account_info(battle_history_info)?;

    let alive = args.index == 0;
    let mut check = check_soul_recycle(args.clone(), alive);

    if alive {
        msg!("Burn Token");
        spl_token_burn_quick(
            mint_info.clone(),
            signer_info.clone(),
            token_program_info.clone(),
            token_account_info.clone(),
        )?;
    } else {
        let index = args.index - 1;
        let mut bodies = battle_history.bodies.clone();
        let body = &bodies[index as usize];
        if body[0] == args.race &&
            body[1] == args.level &&
            body[2] == args.gender &&
            check_body_array(body.clone(), args.enemy_feature.clone()) {
            bodies.remove(index as usize);
            battle_history.bodies = bodies;
        } else {
            return ferror!("Invalid body.");
        }
    }

    battle_history.soul += check;
    battle_history.serialize(&mut *battle_history_info.try_borrow_mut_data()?)?;

    //send fund
    send_fund_to_target(program_id, admin_fund_info.as_ref().cloned(), &signer_info, 0)?;

    Ok(())
}
