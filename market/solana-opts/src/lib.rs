use std::convert::TryInto;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

mod error;
use error::OptError;

pub fn now_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(OptError::InvalidSigner.into())
    } else {
        Ok(())
    }
}

pub fn assert_derivation(program_id: &Pubkey, account: &AccountInfo, path: &[&[u8]]) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if key != *account.key {
        Err(OptError::InvalidDerivedKey.into())
    } else {
        Ok(bump)
    }
}

pub fn assert_rent_exempt(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        Err(OptError::NotRentExempt.into())
    } else {
        Ok(())
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(OptError::InvalidOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_data_empty(account: &AccountInfo) -> ProgramResult {
    if account.data_is_empty() {
        Ok(())
    } else {
        Err(OptError::InvalidDataNotEmpty.into())
    }
}

pub fn assert_eq_pubkey(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(OptError::InvalidEqPubkey.into())
    } else {
        Ok(())
    }
}

pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(OptError::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn assert_associated_address(token_mint: &Pubkey, user_wallet: &Pubkey, dest: &Pubkey) -> ProgramResult {
    let calc = spl_associated_token_account::get_associated_token_address(&user_wallet, &token_mint);
    if calc != *dest {
        Err(OptError::InvalidAssociatedAddress.into())
    } else {
        Ok(())
    }
}

pub fn assert_mint_metadata(mint_pubkey: &Pubkey, metadata_pubkey: &Pubkey) -> ProgramResult {
    let program_id = metaplex_token_metadata::id();
    let path = &[
        metaplex_token_metadata::state::PREFIX.as_bytes(),
        program_id.as_ref(),
        mint_pubkey.as_ref(),
    ];
    let (key, _) = Pubkey::find_program_address(path, &program_id);

    if key != *metadata_pubkey {
        Err(ProgramError::BorshIoError("invalid metadata derivation".to_string()))
    } else {
        Ok(())
    }
}

/// Create account almost from scratch, lifted from
/// https://github.com/solana-labs/solana-program-library/blob/7d4873c61721aca25464d42cc5ef651a7923ca79/associated-token-account/program/src/processor.rs#L51-L98
#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    let rent = &Rent::from_account_info(&rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        &[new_account_info.clone(), system_program_info.clone()],
        &[signer_seeds],
    )?;
    msg!("Completed assignation!");

    Ok(())
}

/// Create a new SPL token account.
#[inline(always)]
pub fn spl_token_create_account<'a>(
    token_program: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    create_account_seeds: &[&[u8]],     // when account is not a pda, is null
    initialize_account_seeds: &[&[u8]], // when account is not a pda, is null
    rent_info: &AccountInfo<'a>,
) -> ProgramResult {
    let size = spl_token::state::Account::LEN;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("spl_token_create_account create");
    invoke_signed(
        &system_instruction::create_account(
            payer_info.key,
            new_account.key,
            required_lamports,
            size as u64,
            token_program.key,
        ),
        &[payer_info.clone(), new_account.clone()],
        &[create_account_seeds],
    )?;

    msg!("spl_token_create_account initialize");
    invoke_signed(
        &spl_token::instruction::initialize_account(token_program.key, new_account.key, mint_info.key, authority.key)?,
        &[
            token_program.clone(),
            new_account.clone(),
            mint_info.clone(),
            authority.clone(),
            rent_info.clone(),
        ],
        &[initialize_account_seeds],
    )?;
    msg!("spl_token_create_account success");

    Ok(())
}

pub fn spl_token_transfer<'a>(
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[&signer_seeds],
    )
}

#[inline(always)]
pub fn create_or_allocate_account_raw_invoke<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
) -> Result<(), ProgramError> {
    let rent = &Rent::from_account_info(&rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    msg!("Allocate space for the account");
    invoke(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program_info.clone()],
    )?;

    msg!("Assign the account to the owning program");
    invoke(
        &system_instruction::assign(new_account_info.key, &program_id),
        &[new_account_info.clone(), system_program_info.clone()],
    )?;
    msg!("Completed assignation!");

    Ok(())
}

pub fn spl_token_transfer_invoke<'a>(
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
    )
}
