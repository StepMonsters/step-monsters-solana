use mpl_token_metadata::instruction::update_metadata_accounts_v2;
use mpl_token_metadata::state::{DataV2, Metadata, TokenMetadataAccount};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use solana_program::program::invoke_signed;

use crate::{state::*, utils::*};

pub fn process_update_metadata(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let metadata = Metadata::from_account_info(metadata_info)?;

    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;
    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let creators = vec![
        mpl_token_metadata::state::Creator {
            address: *pda_creator_info.key,
            verified: true,
            share: 100,
        },
    ];

    let data = metadata.data.clone();
    let data_v2 = DataV2 {
        name: data.name.clone(),
        symbol: data.symbol.clone(),
        uri: data.uri.clone(),
        seller_fee_basis_points: data.seller_fee_basis_points.clone(),
        creators: Some(creators.clone()),
        collection: metadata.collection,
        uses: metadata.uses,
    };

    msg!("Update Metadata");
    invoke_signed(
        &update_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *pda_creator_info.key,
            Some(*pda_creator_info.key),
            Some(data_v2.clone()),
            Some(true),
            Some(true),
        ),
        &[
            signer_info.clone(),
            metadata_info.clone(),
            pda_creator_info.clone(),
            metadata_program_info.clone(),
        ],
        &[&pda_seed],
    )?;

    Ok(())
}
