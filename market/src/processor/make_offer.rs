use crate::{state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction, sysvar,
};

pub fn process_make_offer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: MakeOfferArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let nft_info = next_account_info(account_info_iter)?;
    let nft_return_info = next_account_info(account_info_iter)?;
    let new_offer_info = next_account_info(account_info_iter)?; //new offer info
    let bid_store_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_bid_store(&program_id, &new_offer_info, &bid_store_info)?;
    assert_data_empty(&new_offer_info)?;

    // create new_offer_info
    create_or_allocate_account_raw_invoke(
        *program_id,
        &new_offer_info,
        &rent_info,
        &system_info,
        &signer_info,
        OfferData::LEN,
    )?;

    //transfer sol to bid store
    invoke(
        &system_instruction::transfer(&signer_info.key, &bid_store_info.key, args.price as u64),
        &[
            signer_info.clone(),
            bid_store_info.clone(),
            system_info.clone(),
        ],
    )?;

    let mut offer_data = OfferData::from_account_info(&new_offer_info)?;

    offer_data.offerer = *signer_info.key;
    offer_data.nft = *nft_info.key;
    offer_data.nft_return = *nft_return_info.key;
    offer_data.price = args.price;
    offer_data.serialize(&mut &mut new_offer_info.data.borrow_mut()[..])?;
    Ok(())
}
