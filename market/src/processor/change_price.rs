use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_change_price(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ChangePriceArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let auction_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_owned_by(&auction_info, &program_id)?;

    let mut auction_data = AuctionData::from_account_info(auction_info)?;

    if auction_data.is_initialized() == false
        || auction_data.is_claim == true
        || auction_data.last_bid.is_some()
    {
        return ferror!("invalid state");
    }
    if auction_data.creator != *signer_info.key {
        return ferror!("invalid authority");
    }

    auction_data.price = Some(args.price);
    auction_data.serialize(&mut &mut auction_info.data.borrow_mut()[..])?;

    Ok(())
}
