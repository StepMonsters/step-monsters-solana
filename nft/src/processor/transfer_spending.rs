use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use solana_program::program::{invoke, invoke_signed};
use spl_associated_token_account::instruction::create_associated_token_account;

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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferSpendingArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let token_admin_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    //check authority
    let config_info = next_account_info(account_info_iter)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }
    assert_config(&program_id, &config_info)?;
    assert_owned_by(config_info, &program_id)?;

    assert_signer(&signer_info)?;

    msg!("Token Admin Seeds");
    let bump_seed = assert_derivation(
        program_id,
        token_admin_info,
        &[
            SEED_TOKEN_ADMIN.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let token_admin_seeds = [
        SEED_TOKEN_ADMIN.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];

    msg!("Transfer token to signer");
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program_info.key,
            program_ata_info.key,
            signer_ata_info.key,
            token_admin_info.key,
            &[token_admin_info.key],
            args.amount,
        )?,
        &[
            program_ata_info.clone(),
            signer_ata_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            token_admin_info.clone()
        ],
        &[&token_admin_seeds],
    )?;

    Ok(())
}

pub fn process_transfer_from_spending_temp(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferSpendingArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let target_info = next_account_info(account_info_iter)?;
    let target_ata_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    if target_ata_info.lamports() <= 0 {
        invoke(
            &create_associated_token_account(
                signer_info.key,
                target_info.key,
                mint_info.key,
                token_program_info.key,
            ),
            &[
                signer_info.clone(),
                signer_ata_info.clone(),
                target_info.clone(),
                target_ata_info.clone(),
                mint_info.clone(),
                ass_token_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
            ],
        )?;
    };

    let amount = args.amount;

    spl_token_transfer_invoke(
        token_program_info.clone(),
        signer_ata_info.clone(),
        target_ata_info.clone(),
        signer_info.clone(),
        amount,
    )?;

    Ok(())
}
