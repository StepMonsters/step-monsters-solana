use crate::{state::*, utils::*};
use borsh::BorshSerialize;
use metaplex_token_metadata::state::Metadata;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, sysvar,
};

pub fn process_accept_offer(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let charge_addr_info = next_account_info(account_info_iter)?;
    let nft_account_info = next_account_info(account_info_iter)?;
    let nft_return_info = next_account_info(account_info_iter)?;
    let nft_metadata_info = next_account_info(account_info_iter)?;
    let offerer_info = next_account_info(account_info_iter)?;
    let new_offer_info = next_account_info(account_info_iter)?; //offer info
    let bid_store_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_config(&program_id, &config_info)?;
    assert_eq_pubkey(&charge_addr_info, &config_data.charge_addr)?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    let bid_store_bump = assert_bid_store(&program_id, &new_offer_info, &bid_store_info)?;

    let bid_store_seed = [
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        new_offer_info.key.as_ref(),
        "bid_store".as_bytes(),
        &[bid_store_bump],
    ];

    let mut offer_data = OfferData::from_account_info(&new_offer_info)?;
    assert_eq_pubkey(&offerer_info, &offer_data.offerer)?;
    assert_eq_pubkey(&nft_return_info, &offer_data.nft_return)?;

    // transfer nft to offerer
    spl_token_transfer_invoke(
        spl_token_info.clone(),
        nft_account_info.clone(),
        nft_return_info.clone(),
        signer_info.clone(),
        1,
    )?;

    assert_mint_metadata(&offer_data.nft, nft_metadata_info.key)?;
    let nft_metadata = Metadata::from_account_info(nft_metadata_info)?;
    let mut royalty_amt = 0;

    let price = offer_data.price;
    //transfer sol to creators
    if nft_metadata.data.seller_fee_basis_points > 0 {
        royalty_amt = price
            .checked_mul(nft_metadata.data.seller_fee_basis_points as u64)
            .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?
            .checked_div(10000 as u64)
            .ok_or(ProgramError::BorshIoError("royalty_amt cal error".into()))?;
        for creator in nft_metadata.data.creators.unwrap().iter() {
            let nft_creator_info = next_account_info(account_info_iter)?;
            // metadata make sure all share = 100
            let amount = royalty_amt
                .checked_mul(creator.share as u64)
                .ok_or(ProgramError::InvalidArgument)?
                .checked_div(100 as u64)
                .ok_or(ProgramError::InvalidArgument)?;
            if amount > 0 {
                invoke_signed(
                    &system_instruction::transfer(
                        &bid_store_info.key,
                        &nft_creator_info.key,
                        amount,
                    ),
                    &[
                        bid_store_info.clone(),
                        nft_creator_info.clone(),
                        system_info.clone(),
                    ],
                    &[&bid_store_seed],
                )?;
            }
        }
    }

    //transfer sol to fee reciver
    let fee_amt = price
        .checked_mul(config_data.charge_rate as u64)
        .ok_or(ProgramError::BorshIoError("fee_amt cal error".into()))?
        .checked_div(10000 as u64)
        .ok_or(ProgramError::BorshIoError("fee_amt cal error".into()))?;
    invoke_signed(
        &system_instruction::transfer(&bid_store_info.key, &charge_addr_info.key, fee_amt),
        &[
            bid_store_info.clone(),
            charge_addr_info.clone(),
            system_info.clone(),
        ],
        &[&bid_store_seed],
    )?;

    //transfer sol to seller
    invoke_signed(
        &system_instruction::transfer(
            &bid_store_info.key,
            &signer_info.key,
            offer_data
                .price
                .checked_sub(royalty_amt)
                .ok_or(ProgramError::BorshIoError("seller_amt cal error".into()))?
                .checked_sub(fee_amt)
                .ok_or(ProgramError::BorshIoError("seller_amt cal error".into()))?,
        ),
        &[
            bid_store_info.clone(),
            signer_info.clone(),
            system_info.clone(),
        ],
        &[&bid_store_seed],
    )?;

    config_data.total_trade += price;
    offer_data.is_done = true;
    offer_data.serialize(&mut &mut new_offer_info.data.borrow_mut()[..])?;
    Ok(())
}
