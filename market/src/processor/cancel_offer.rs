use crate::{ferror, state::*, utils::*, PREFIX};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction, sysvar,
    msg,
    program_error::ProgramError,
};

pub fn process_cancel_offer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let nft_info = next_account_info(account_info_iter)?;
    let new_offer_info = next_account_info(account_info_iter)?; // offer info
    let bid_store_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;

    let mut offer_data = OfferData::from_account_info(&new_offer_info)?;
    assert_eq_pubkey(&signer_info, &offer_data.offerer)?;
    assert_eq_pubkey(&nft_info, &offer_data.nft)?;

    if offer_data.is_done || offer_data.is_canceled {
        return ferror!("invalid cancel state");
    }

    let bid_store_bump = assert_bid_store(&program_id, &new_offer_info, &bid_store_info)?;
    let bid_store_seed = [
        PREFIX.as_bytes(),
        program_id.as_ref(),
        new_offer_info.key.as_ref(),
        "bid_store".as_bytes(),
        &[bid_store_bump],
    ];

    invoke_signed(
        &system_instruction::transfer(
            &bid_store_info.key,
            &signer_info.key,
            offer_data.price,
        ),
        &[
            bid_store_info.clone(),
            signer_info.clone(),
            system_info.clone(),
        ],
        &[&bid_store_seed],
    )?;


    offer_data.is_canceled = true;
    offer_data.serialize(&mut &mut new_offer_info.data.borrow_mut()[..])?;
    Ok(())
}
