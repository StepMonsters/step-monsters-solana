use borsh::BorshSerialize;
use metaplex_token_metadata::state::Metadata;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    log,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar,
};
use spl_token::state::Account;
use crate::{ferror, state::*, utils::*, PREFIX};

pub fn process_place_bid(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: PlaceBidArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    // add config_info charge_addr_info
    let charge_addr_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    // add user_info
    let user_info = next_account_info(account_info_iter)?;
    let auction_creator_user_info = next_account_info(account_info_iter)?;
    let auction_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let bid_info = next_account_info(account_info_iter)?;
    let bid_store_info = next_account_info(account_info_iter)?;
    let auction_creator_info = next_account_info(account_info_iter)?;
    let nft_store_info = next_account_info(account_info_iter)?;
    let nft_return_info = next_account_info(account_info_iter)?;
    let last_bidder_info = next_account_info(account_info_iter)?;
    let nft_metadata_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_config(&program_id, &config_info)?;
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&charge_addr_info, &config_data.charge_addr)?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_owned_by(&auction_info, &program_id)?;
    let auth_bump = assert_auction_authority(&program_id, &auction_info, &authority_info)?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;
    let bid_store_bump = assert_bid_store(&program_id, &auction_info, &bid_store_info)?;
    let bid_store_seed = [
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_info.key.as_ref(),
        "bid_store".as_bytes(),
        &[bid_store_bump],
    ];
    let mut auction_data = AuctionData::from_account_info(auction_info)?;
    if auction_data.is_initialized() == false {
        return ferror!("invalid auction bid");
    }

    assert_eq_pubkey(&bid_store_info, &auction_data.bid_store)?;
    assert_user_info(
        &program_id,
        &auction_creator_info,
        &auction_creator_user_info,
    )?;

    let user_bump = assert_user_info(&program_id, &signer_info, &user_info)?;
    let bid_bump = assert_bid_data(&program_id, &auction_info, &signer_info, &bid_info)?;

    
    let bid_price: u64;

    //create user_info
    if user_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            user_info,
            rent_info,
            system_info,
            signer_info,
            UserInfo::LEN,
            &[
                program_id.as_ref(),
                signer_info.key.as_ref(),
                "user_info".as_bytes(),
                &[user_bump],
            ],
        )?;
    }

    let mut user_data = UserInfo::from_account_info(user_info)?;
    let mut auction_user_data = UserInfo::from_account_info(auction_creator_user_info)?;
    //create bid_info
    if bid_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            bid_info,
            rent_info,
            system_info,
            signer_info,
            BidData::LEN,
            &[
                PREFIX.as_bytes(),
                program_id.as_ref(),
                auction_info.key.as_ref(),
                signer_info.key.as_ref(),
                "bid".as_bytes(),
                &[bid_bump],
            ],
        )?;
    }
    let mut bid_data = BidData::from_account_info(bid_info)?;

        if auction_data.is_claim == true || auction_data.last_bid.is_some() {
            return ferror!("FixedPriceSale invalid state");
        }
        let fixed_price = auction_data.price.unwrap();
        if fixed_price != args.price {
            return ferror!("FixedPriceSale invalid price");
        }
        bid_price = fixed_price;

        //transfer NFT
        spl_token_transfer(
            spl_token_info.clone(),
            nft_store_info.clone(),
            nft_return_info.clone(),
            authority_info.clone(),
            1,
            &[
                crate::PREFIX.as_bytes(),
                program_id.as_ref(),
                auction_info.key.as_ref(),
                "authority".as_bytes(),
                &[auth_bump],
            ],
        )?;
        assert_mint_metadata(&auction_data.nft_mint, nft_metadata_info.key)?;
        let nft_metadata = Metadata::from_account_info(nft_metadata_info)?;
        let mut royalty_amt = 0;
        //transfer sol to creators
        if nft_metadata.data.seller_fee_basis_points > 0 {
            royalty_amt = fixed_price
                .checked_mul(nft_metadata.data.seller_fee_basis_points as u64)
                .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?
                .checked_div(10000 as u64)
                .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?;
            for creator in nft_metadata.data.creators.unwrap().iter() {
                let nft_creator_info = next_account_info(account_info_iter)?;
                // msg!("nft_creator_info-----{:?}", nft_creator_info);
                // metadata make sure all share = 100
                let amount = royalty_amt
                    .checked_mul(creator.share as u64)
                    .ok_or(ProgramError::InvalidArgument)?
                    .checked_div(100 as u64)
                    .ok_or(ProgramError::InvalidArgument)?;
                if amount > 0 {
                    // msg!("royalty transfer  {} {}", &creator.address, amount);
                    invoke(
                        &system_instruction::transfer(&signer_info.key, &creator.address, amount),
                        &[
                            signer_info.clone(),
                            nft_creator_info.clone(),
                            system_info.clone(),
                        ],
                    )?;
                }
            }
        }
        //transfer sol to fee reciver
        let fee_amt = fixed_price
            .checked_mul(config_data.charge_rate as u64)
            .ok_or(ProgramError::BorshIoError("fee_amt cal error".into()))?
            .checked_div(10000 as u64)
            .ok_or(ProgramError::BorshIoError("fee_amt cal error".into()))?;
        invoke(
            &system_instruction::transfer(&signer_info.key, &charge_addr_info.key, fee_amt),
            &[
                signer_info.clone(),
                charge_addr_info.clone(),
                system_info.clone(),
            ],
        )?;

        //transfer sol to seller
        invoke(
            &system_instruction::transfer(
                &signer_info.key,
                &auction_data.creator,
                bid_price
                    .checked_sub(royalty_amt)
                    .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?
                    .checked_sub(fee_amt)
                    .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?,
            ),
            &[
                signer_info.clone(),
                auction_creator_info.clone(),
                system_info.clone(),
            ],
        )?;

        auction_data.is_claim = true;
        bid_data.is_done = true;
        bid_data.amount = args.price;
        bid_data.bidder = *signer_info.key;
        // mine logic
        //deal with config_data

        config_data.total_trade += fixed_price;
        // deal with user_ info
        user_data.total_trade += fixed_price;
        //deal with auction creator info

        auction_user_data.total_trade += fixed_price;

    

    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;
    auction_user_data.serialize(&mut &mut auction_creator_user_info.data.borrow_mut()[..])?;
    user_data.serialize(&mut &mut user_info.data.borrow_mut()[..])?;
    bid_data.serialize(&mut &mut bid_info.data.borrow_mut()[..])?;
    auction_data.last_bid = Some(bid_data);
    auction_data.serialize(&mut &mut auction_info.data.borrow_mut()[..])?;
    Ok(())
}
