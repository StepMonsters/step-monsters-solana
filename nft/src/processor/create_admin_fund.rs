use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{utils::*};
use crate::utils_mint::create_admin_fund_info;

pub fn process_create_admin_fund(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let admin_fund_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    if admin_fund_info.lamports() <= 0 {
        create_admin_fund_info(
            &program_id,
            &admin_fund_info,
            &rent_info,
            &system_info,
            &signer_info,
        )?;
    };

    Ok(())
}
