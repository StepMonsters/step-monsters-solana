use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use solana_program::program::invoke;

use crate::{ferror, state::*, utils::*};

pub fn process_transfer_to_spending(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferSpendingArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let spending_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create Spending Wallet Account");
    if spending_info.lamports() <= 0 {
        let bump_seed = assert_spending(&program_id, &spending_info, &signer_info)?;
        let spending_seeds = &[
            SEED_STEP_MONSTER.as_bytes(),
            program_id.as_ref(),
            "spending".as_bytes(),
            signer_info.key.as_ref(),
            &[bump_seed],
        ];
        create_or_allocate_account_raw(
            *program_id,
            spending_info,
            rent_info,
            system_info,
            signer_info,
            SpendingAccount::LEN,
            spending_seeds,
        )?;
    }

    msg!("Check Amount");
    let mut amount = args.amount.clone();
    let lamports = signer_info.lamports().clone();
    if amount >= lamports {
        return ferror!("Invalid amount.");
    }
    if amount <= 1_000_000 {
        amount -= 10_000;
    } else if lamports - amount <= 1_000_000 {
        amount -= 1_000_000;
    }

    msg!("Transfer To Spending Account");
    invoke(
        &system_instruction::transfer(
            signer_info.key,
            spending_info.key,
            amount),
        &[
            signer_info.clone(),
            spending_info.clone(),
            system_info.clone(),
        ],
    )?;

    msg!("Serialize Spending Account");
    let mut spending = SpendingAccount::from_account_info(spending_info)?;
    spending.amount += amount;
    spending.serialize(&mut *spending_info.try_borrow_mut_data()?)?;

    Ok(())
}

pub fn process_transfer_from_spending(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferSpendingArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let spending_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Transfer To Spending Account");
    invoke(
        &system_instruction::transfer(
            spending_info.key,
            signer_info.key,
            args.amount),
        &[
            signer_info.clone(),
            spending_info.clone(),
            system_info.clone(),
        ],
    )?;

    msg!("Serialize Spending Account");
    let mut spending = SpendingAccount::from_account_info(spending_info)?;
    spending.amount -= args.amount;
    spending.serialize(&mut *spending_info.try_borrow_mut_data()?)?;

    Ok(())
}
