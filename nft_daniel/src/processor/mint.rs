use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction,
    sysvar,
};

use crate::{state::*, utils::*};

pub fn process_mint(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_recevier_info = next_account_info(account_info_iter)?; // fee_recevier: wallet
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    
    let metadata_program_info = next_account_info(account_info_iter)?;    
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    
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
    
    // if pda_creator_info.data_is_empty() {
    //     create_or_allocate_account_raw(
    //         *program_id,
    //         pda_creator_info,
    //         rent_info,
    //         system_info,
    //         signer_info,
    //         ConfigureData::LEN,
    //         &pda_seed,
    //     )?;
    // }

    let config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_recevier_info, &config_data.fee_recevier)?;

    // mint fee
    if config_data.price > 0 {
        invoke(
            &system_instruction::transfer(&signer_info.key, &config_data.fee_recevier, config_data.price),
            &[
                signer_info.clone(),
                fee_recevier_info.clone(),
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

    Ok(())
}
