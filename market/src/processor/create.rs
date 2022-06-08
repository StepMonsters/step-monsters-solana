use borsh::BorshSerialize;
use metaplex_token_metadata::state::Metadata;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{self},
};
use spl_token::state::Mint;

use crate::{ferror, state::*, utils::*};

pub fn process_create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let nft_creator_info = next_account_info(account_info_iter)?;
    let nft_creator_data_info = next_account_info(account_info_iter)?;
    let new_auction_info = next_account_info(account_info_iter)?; // save AuctionData, create but not init
    let authority_info = next_account_info(account_info_iter)?; // [PREFIX, program_id, new_auction, 'authority']
    let nft_mint_info = next_account_info(account_info_iter)?; // NFT mint address
    let nft_metadata_info = next_account_info(account_info_iter)?; // nft metadata account, [PREFIX, program_id, mint_pk]
    let nft_account_info = next_account_info(account_info_iter)?; // account own the nft has been approve for authority
    let nft_store_info = next_account_info(account_info_iter)?; // owned by authority_info to keep NFT
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // check assert
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;
    assert_creator_data(&program_id, &nft_creator_info, &nft_creator_data_info)?;
    assert_signer(&new_auction_info)?;
    assert_data_empty(&new_auction_info)?;
    let auth_bump = assert_auction_authority(&program_id, &new_auction_info, &authority_info)?;
    assert_mint_metadata(nft_mint_info.key, nft_metadata_info.key)?;
    let nft_store_bump = assert_nft_store(&program_id, &new_auction_info, &nft_store_info)?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;

    // check config is allow to create auction
    let config_account = ConfigureData::from_account_info(config_info)?;
    if config_account.is_initialized != true {
        return ferror!("auction not initialized");
    }

    // check nft mint and metadata
    let nft_mint = Mint::unpack_unchecked(&nft_mint_info.data.borrow())?;
    if nft_mint.supply != 1 || nft_mint.decimals != 0 || nft_mint.is_initialized != true {
        return ferror!("invalid nft_mint");
    }
    let nft_metadata = Metadata::from_account_info(nft_metadata_info)?;
    if nft_metadata.key != metaplex_token_metadata::state::Key::MetadataV1
        || nft_metadata.mint != *nft_mint_info.key
    {
        return ferror!("invalid nft_metadata");
    }
    if nft_metadata.data.seller_fee_basis_points + config_account.charge_rate as u16 >= 10000 {
        return ferror!("charge hurts buyer very much");
    }
    let mut is_creator = false;
    if nft_metadata.data.creators.is_some() {
        for creator in nft_metadata.data.creators.unwrap().iter() {
            if creator.verified == true && creator.address == *nft_creator_info.key {
                let nft_creator_data_account =
                    SetCreatorWhitelistData::from_account_info(nft_creator_data_info)?;
                if nft_creator_data_account.is_activated != true {
                    return ferror!("creator is not activated");
                }
                is_creator = true
            }
        }
    }
    if is_creator == false {
        return ferror!("invalid nft or creator");
    }

    let authority_seed = [
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        new_auction_info.key.as_ref(),
        "authority".as_bytes(),
        &[auth_bump],
    ];

    // create nft_store_info
    spl_token_create_account(
        &spl_token_info,
        &signer_info,
        &nft_mint_info,
        &nft_store_info,
        &authority_info,
        &[
            crate::PREFIX.as_bytes(),
            program_id.as_ref(),
            new_auction_info.key.as_ref(),
            "nft_store".as_bytes(),
            &[nft_store_bump],
        ],
        &authority_seed,
        &rent_info,
    )?;

    // create new_auction_info
    create_or_allocate_account_raw_invoke(
        *program_id,
        &new_auction_info,
        &rent_info,
        &system_info,
        &signer_info,
        AuctionData::LEN,
    )?;

    let mut auction_data = AuctionData::from_account_info(&new_auction_info)?;
    let now_ts = now_timestamp();
    let begin_ts = if args.begin_ts.is_none() {
        Some(now_ts)
    } else {
        if args.begin_ts.unwrap() < now_ts {
            return ferror!("invalid begin_ts");
        }
        args.begin_ts
    };


    let index = now_ts / 86400;
    auction_data.timestamp = index * 86400; //easy to get daily auction_data
    auction_data.is_init = true;
    auction_data.creator = *signer_info.key;
    auction_data.nft_mint = *nft_mint_info.key;
    auction_data.nft_store = *nft_store_info.key;
    auction_data.begin_ts = begin_ts;
    auction_data.duration = None;
    auction_data.is_claim = false;
    auction_data.last_bid = None;
    auction_data.price = args.price;

    spl_token_transfer_invoke(
        spl_token_info.clone(),
        nft_account_info.clone(),
        nft_store_info.clone(),
        signer_info.clone(),
        1,
    )?;

    auction_data.serialize(&mut &mut new_auction_info.data.borrow_mut()[..])?;

    Ok(())
}
