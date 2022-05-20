use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_cancel(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let auction_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let nft_store_info = next_account_info(account_info_iter)?;
    let nft_return_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_owned_by(&auction_info, &program_id)?;
    let auth_bump = assert_auction_authority(&program_id, &auction_info, &authority_info)?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;

    let mut auction_data = AuctionData::from_account_info(auction_info)?;
    if auction_data.is_initialized() == false
        || auction_data.is_claim == true
        || auction_data.last_bid.is_some()
    {
        return ferror!("invalid cancel state");
    }
    if auction_data.creator != *signer_info.key || auction_data.nft_store != *nft_store_info.key {
        return ferror!("invalid cancel authority");
    }

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

    auction_data.is_claim = true;
    auction_data.serialize(&mut &mut auction_info.data.borrow_mut()[..])?;

    Ok(())
}
