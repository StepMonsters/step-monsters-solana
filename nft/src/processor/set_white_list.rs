use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};

pub fn process_set_white_list(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let white_list_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create White List Info");
    let bump_seed = assert_derivation(
        program_id,
        white_list_info,
        &[
            SEED_WHITELIST.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let white_list_seeds = &[
        SEED_WHITELIST.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        white_list_info,
        rent_info,
        system_info,
        signer_info,
        MAX_WHITE_LIST_LENGTH,
        white_list_seeds,
    )?;

    let mut whitelist = WhiteList::from_account_info(white_list_info)?;
    whitelist.name = 1;
    whitelist.serialize(&mut *white_list_info.try_borrow_mut_data()?)?;

    Ok(())
}
