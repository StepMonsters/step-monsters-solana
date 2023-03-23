use borsh::BorshSerialize;
use metaplex_token_metadata::state::Metadata;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, sysvar,
};
use spl_associated_token_account::create_associated_token_account;

use crate::{ferror, PREFIX, state::*, utils::*};

pub fn process_place_bid(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: PlaceBidArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let charge_addr_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let auction_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let bid_info = next_account_info(account_info_iter)?;
    let auction_creator_info = next_account_info(account_info_iter)?;
    let auction_creator_pda_info = next_account_info(account_info_iter)?;
    let signer_lst_pda_info = next_account_info(account_info_iter)?;
    let token_mint_lst_info = next_account_info(account_info_iter)?;
    let nft_store_info = next_account_info(account_info_iter)?;
    let nft_mint_info = next_account_info(account_info_iter)?;
    let nft_return_info = next_account_info(account_info_iter)?;
    let nft_metadata_info = next_account_info(account_info_iter)?;
    let nft_creator_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;

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
    let mut auction_data = AuctionData::from_account_info(auction_info)?;
    if auction_data.is_initialized() == false {
        return ferror!("invalid auction bid");
    }

    let bid_bump = assert_bid_data(&program_id, &auction_info, &signer_info, &bid_info)?;
    let bid_price: u64;

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

    //create nft return info
    if nft_return_info.lamports() <= 0 {
        invoke(
            &create_associated_token_account(
                signer_info.key,
                signer_info.key,
                nft_mint_info.key,
            ),
            &[
                signer_info.clone(),
                nft_return_info.clone(),
                ass_token_program_info.clone(),
                nft_mint_info.clone(),
                spl_token_info.clone(),
                system_info.clone(),
                rent_info.clone()
            ],
        )?;
    }

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

    if args.price_type == 0 {
        //transfer sol to creators
        if nft_metadata.data.seller_fee_basis_points > 0 {
            royalty_amt = fixed_price
                .checked_mul(nft_metadata.data.seller_fee_basis_points as u64)
                .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?
                .checked_div(10000 as u64)
                .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?;
            if royalty_amt > 0 {
                // msg!("royalty transfer  {} {}", &creator.address, amount);
                invoke(
                    &system_instruction::transfer(&signer_info.key, &nft_creator_info.key, royalty_amt),
                    &[
                        signer_info.clone(),
                        nft_creator_info.clone(),
                        system_info.clone(),
                    ],
                )?;
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
                    .ok_or(ProgramError::BorshIoError("seller_amt cal error".into()))?
                    .checked_sub(fee_amt)
                    .ok_or(ProgramError::BorshIoError("seller_amt cal error".into()))?,
            ),
            &[
                signer_info.clone(),
                auction_creator_info.clone(),
                system_info.clone(),
            ],
        )?;
    } else {
        if auction_creator_pda_info.lamports() <= 0 {
            invoke(
                &create_associated_token_account(
                    signer_info.key,
                    auction_creator_info.key,
                    token_mint_lst_info.key,
                ),
                &[
                    signer_info.clone(),
                    auction_creator_info.clone(),
                    auction_creator_pda_info.clone(),
                    token_mint_lst_info.clone(),
                    ass_token_program_info.clone(),
                    spl_token_info.clone(),
                    system_info.clone()
                ],
            )?;
        }
        spl_token_transfer_invoke(
            spl_token_info.clone(),
            signer_lst_pda_info.clone(),
            auction_creator_pda_info.clone(),
            signer_info.clone(),
            args.price,
        )?;
    }

    auction_data.is_claim = true;
    bid_data.is_done = true;
    bid_data.amount = args.price;
    bid_data.bidder = *signer_info.key;
    config_data.total_trade += fixed_price;

    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;
    bid_data.serialize(&mut &mut bid_info.data.borrow_mut()[..])?;
    auction_data.last_bid = Some(bid_data);
    auction_data.serialize(&mut &mut auction_info.data.borrow_mut()[..])?;

    Ok(())
}
