use borsh::BorshSerialize;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar,
};

use crate::{ferror, state::*, utils::*};

pub fn process_mint(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_receiver_info = next_account_info(account_info_iter)?; // fee_receiver: wallet
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;

    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;

    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    // mint fee
    if config_data.price > 0 {
        invoke(
            &system_instruction::transfer(&signer_info.key, &config_data.fee_receiver, config_data.price),
            &[
                signer_info.clone(),
                fee_receiver_info.clone(),
                system_info.clone(),
            ],
        )?;
    }

    let creators = vec![
        mpl_token_metadata::state::Creator {
            address: *pda_creator_info.key,
            verified: true,
            share: 0,
        },
        mpl_token_metadata::state::Creator {
            address: config_data.creator,
            verified: false,
            share: 100,
        },
    ];

    msg!("Create metadata");
    invoke_signed(
        &create_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *pda_creator_info.key, //pda must be signer
            config_data.name,
            config_data.symbol,
            config_data.uri,
            Some(creators),
            config_data.fee,
            true,
            true,
            None,
            None,
        ),
        &[
            metadata_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            pda_creator_info.clone(),
        ],
        &[&pda_seed],
    )?;

    msg!("Create Master Edition");
    invoke_signed(
        &create_master_edition_v3(
            *metadata_program_info.key,
            *edition_info.key,
            *mint_info.key,
            *pda_creator_info.key,
            *signer_info.key,
            *metadata_info.key,
            *signer_info.key,
            Some(1),
        ),
        &[
            edition_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            pda_creator_info.clone(),
        ],
        &[&pda_seed],
    )?;

    msg!("Create Monster Info");
    let bump_seed = assert_derivation(
        program_id,
        monster_info,
        &[
            SEED_MONSTER.as_bytes(),
            program_id.as_ref(),
            &mint_info.key.as_ref(),
        ],
    )?;
    let monster_seeds = &[
        SEED_MONSTER.as_bytes(),
        program_id.as_ref(),
        &mint_info.key.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        monster_info,
        rent_info,
        system_info,
        signer_info,
        MAX_MONSTER_LENGTH,
        monster_seeds,
    )?;
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.race = 1;
    monster.hatch_time = now_timestamp() + 10;
    monster.monster_feature = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
