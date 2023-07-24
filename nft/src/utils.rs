use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::io::Error;

use borsh::BorshDeserialize;
use mpl_token_metadata::error::MetadataError;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

use crate::error::AppError;
use crate::instruction::call_send_fund;
use crate::state::*;

pub fn assert_eq_pubkey(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(AppError::InvalidEqPubkey.into())
    } else {
        Ok(())
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(AppError::InvalidOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(&path, program_id);
    if key != *account.key {
        return Err(AppError::InvalidDerivedKey.into());
    }
    Ok(bump)
}

pub fn assert_config(program_id: &Pubkey, account: &AccountInfo) -> Result<u8, ProgramError> {
    let path = &[SEED_BATTLE.as_bytes(), program_id.as_ref(), "configure".as_bytes()];
    assert_derivation(&program_id, &account, path)
}


pub fn assert_monster(
    program_id: &Pubkey,
    mint_info: &AccountInfo,
    monster_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_MONSTER.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
    ];
    assert_derivation(&program_id, &monster_info, path)
}

pub fn assert_incubator(
    program_id: &Pubkey,
    nft_mint_info: &AccountInfo,
    incubator_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        nft_mint_info.key.as_ref(),
    ];
    assert_derivation(&program_id, &incubator_info, path)
}

pub fn assert_nft_store(
    program_id: &Pubkey,
    mint_info: &AccountInfo,
    nft_store_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        mint_info.key.as_ref(),
        "nft_store".as_bytes(),
    ];
    assert_derivation(&program_id, &nft_store_info, path)
}

pub fn assert_monster_authority(
    program_id: &Pubkey,
    authority_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "authority".as_bytes(),
    ];
    assert_derivation(&program_id, &authority_info, path)
}

pub fn assert_spending(
    program_id: &Pubkey,
    spending_info: &AccountInfo,
    signer_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_STEP_MONSTER.as_bytes(),
        program_id.as_ref(),
        "spending".as_bytes(),
        signer_info.key.as_ref(),
    ];
    assert_derivation(&program_id, &spending_info, path)
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn get_random(seed: u8) -> Result<u64, ProgramError> {
    let clock = Clock::get()?;
    let mut hasher = DefaultHasher::new();
    hasher.write_u8(seed);
    hasher.write_u64(clock.slot);
    hasher.write_i64(clock.unix_timestamp);
    let mut random_value: [u8; 8] = [0u8; 8];
    random_value.copy_from_slice(&hasher.finish().to_le_bytes()[..8]);
    Ok(u64::from_le_bytes(random_value))
}

pub fn get_random_u8(seed: u8, divisor: u64) -> Result<u8, ProgramError> {
    let random = get_random(seed)?;
    Ok((random % divisor) as u8)
}

pub fn assert_pda_creator(
    program_id: &Pubkey,
    pda_creator_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
    ];
    assert_derivation(&program_id, &pda_creator_info, path)
}

pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
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
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
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
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        &[new_account_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
    )?;
    msg!("Completed assignation!");

    Ok(())
}


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

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if (data[0] != data_type as u8 && data[0] != Key::Uninitialized as u8)
        || data.len() != data_size
    {
        return Err(MetadataError::DataTypeMismatch.into());
    }
    let result: T = try_from_slice_unchecked(data)?;
    Ok(result)
}

pub fn try_from_slice_unchecked<T: BorshDeserialize>(data: &[u8]) -> Result<T, Error> {
    let mut data_mut = data;
    let result = T::deserialize(&mut data_mut)?;
    Ok(result)
}

pub fn game_config_to_vector(data: [[u32; 6]; 10]) -> Vec<Vec<u32>> {
    let mut config = Vec::new();
    for item in data {
        config.push(Vec::from(item));
    }
    return config;
}

pub fn feature_config_to_vector(data: [[u16; 7]; 64]) -> Vec<Vec<u16>> {
    let mut config = Vec::new();
    for item in data {
        config.push(Vec::from(item));
    }
    return config;
}

pub fn check_body_array(mut body: Vec<u8>, attrs: Vec<u8>) -> bool {
    body.remove(0);
    body.remove(0);
    body.remove(0);
    if body.len() != attrs.len() {
        return false;
    }
    for i in 0..body.len() {
        if body[i] != attrs[i] {
            return false;
        }
    }
    return true;
}

pub fn check_soul_recycle(args: RecycleArgs, alive: bool) -> u64 {
    let total = args.hp + args.attack + args.defense +
        args.speed + args.agility + args.efficiency;
    if alive {
        return total * 90 / 100 / 100 * 3;
    } else {
        return total * 90 / 100 / 100;
    }
}

pub fn check_soul_revive(args: ReviveArgs) -> u64 {
    let total = args.hp + args.attack + args.defense +
        args.speed + args.agility + args.efficiency;
    return total * 5 * 150 / 100 / 100;
}

pub fn rarity_formula(mut p: f64, mut min: f64, mut max: f64) -> f64 {
    p *= 100.0_f64;
    min *= 100.0_f64;
    max *= 100.0_f64;
    let a = p.log10() - min.log10();
    let b = max.log10() - min.log10();
    return 1.0_f64 - a / b;
}

pub fn assert_admin_fund_info(
    program_id: &Pubkey,
    fund_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        SEED_ADMIN_FUND_INFO.as_bytes(),
        program_id.as_ref(),
    ];
    assert_derivation(&program_id, &fund_info, path)
}

pub fn send_fund_to_target<'a>(
    program_id: &Pubkey,
    admin_fund_info: Result<&AccountInfo<'a>, &ProgramError>,
    signer_info: &AccountInfo<'a>,
    byte_size: usize,
) -> Result<(), ProgramError> {
    let mut amount = 5000;
    if byte_size > 0 {
        amount = calculate_rent(byte_size);
    };
    if byte_size == MAX_METADATA_ACCOUNT_LENGTH {
        amount += 10_000_000;
    }

    match admin_fund_info {
        Ok(admin_fund_info) => {
            let bump_seed = assert_derivation(
                program_id,
                &admin_fund_info,
                &[
                    SEED_ADMIN_FUND_INFO.as_bytes(),
                    program_id.as_ref(),
                ],
            )?;
            let fund_seeds = [
                SEED_ADMIN_FUND_INFO.as_bytes(),
                program_id.as_ref(),
                &[bump_seed],
            ];

            let send_fund_args = SendFundArgs { amount: amount as u64 };
            let ins = call_send_fund(
                program_id,
                admin_fund_info.key,
                signer_info.key,
                send_fund_args,
            )?;
            invoke_signed(&ins,
                          &[
                              admin_fund_info.clone(),
                              signer_info.clone(),
                          ],
                          &[&fund_seeds],
            )?;
        }
        Err(_) => {}
    }

    Ok(())
}

pub fn calculate_rent(size: usize) -> u64 {
    let rent = Rent::default();
    let rent_exempt_minimum = rent.minimum_balance(size);
    // let rent_sysvar = solana_program::sysvar::rent::Rent::from_account_info(account).unwrap();
    // let rent_exempt_minimum = rent_sysvar.minimum_balance(size);
    rent_exempt_minimum
}
