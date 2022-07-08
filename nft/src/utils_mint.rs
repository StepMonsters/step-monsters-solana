use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2, update_metadata_accounts_v2};
use mpl_token_metadata::state::{Creator, DataV2, Metadata};
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::state::{ConfigureData, MAX_MONSTER_LENGTH, SEED_BATTLE, SEED_MONSTER};
use crate::utils::{assert_derivation, assert_pda_creator, create_or_allocate_account_raw};

pub fn create_monster_info<'a>(
    program_id: &Pubkey,
    monster_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    signer_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
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

    Ok(())
}

pub fn create_metadata_edition<'a>(
    program_id: &Pubkey,
    pda_creator_info: &AccountInfo<'a>,
    config_data: ConfigureData,
    signer_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    metadata_info: &AccountInfo<'a>,
    edition_info: &AccountInfo<'a>,
    metadata_program_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
    rent_info: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
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
            share: 0,
        },
        mpl_token_metadata::state::Creator {
            address: config_data.creator,
            verified: false,
            share: 100,
        },
    ];

    let name_id = String::from(" #") + &config_data.current_id.to_string();
    msg!("Create Metadata");
    invoke_signed(
        &create_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *pda_creator_info.key, //pda must be signer
            config_data.name.clone() + &name_id,
            config_data.symbol.clone(),
            config_data.uri.clone(),
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

pub fn update_metadata<'a>(
    program_id: &Pubkey,
    signer_info: &AccountInfo<'a>,
    metadata_info: &AccountInfo<'a>,
    pda_creator_info: &AccountInfo<'a>,
    metadata_program_info: &AccountInfo<'a>,
    uri_new: String,
) -> Result<(), ProgramError> {
    let metadata = Metadata::from_account_info(metadata_info)?;
    let data = metadata.data;
    let creators = check_creators(
        pda_creator_info,
        data.creators,
    );
    let mut creators_new = creators.clone();
    if uri_new == "null" {
        creators_new.push(
            mpl_token_metadata::state::Creator {
                address: *signer_info.key,
                verified: false,
                share: 0,
            }
        );
    }

    let data_v2 = DataV2 {
        name: data.name,
        symbol: data.symbol,
        uri: uri_new,
        seller_fee_basis_points: data.seller_fee_basis_points,
        creators: Some(creators_new),
        collection: None,
        uses: None,
    };
    let pda_bump = assert_pda_creator(&program_id, pda_creator_info)?;
    let pda_seed = [
        SEED_BATTLE.as_bytes(),
        program_id.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];
    invoke_signed(
        &update_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *pda_creator_info.key,
            Some(*pda_creator_info.key),
            Some(data_v2),
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

fn check_creators(pda_creator_info: &AccountInfo, creators: Option<Vec<Creator>>) -> Vec<Creator> {
    return match creators {
        Some(c) => c,
        None => {
            let creators = vec![
                mpl_token_metadata::state::Creator {
                    address: *pda_creator_info.key,
                    verified: true,
                    share: 0,
                }
            ];
            creators
        }
    };
}