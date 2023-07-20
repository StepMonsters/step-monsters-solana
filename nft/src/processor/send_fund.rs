use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, pubkey::Pubkey};
use solana_program::sysvar::Sysvar;

use crate::{state::*, utils::*};

pub fn process_send_fund(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: SendFundArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let admin_fund_info = next_account_info(account_info_iter)?;
    let signer_info = next_account_info(account_info_iter)?;

    assert_admin_fund_info(program_id, admin_fund_info)?;

    let amount = args.amount;
    **admin_fund_info.lamports.borrow_mut() -= amount;
    **signer_info.lamports.borrow_mut() += amount;

    Ok(())
}
