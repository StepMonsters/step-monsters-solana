use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub use solana_opts::*;

pub fn assert_config(program_id: &Pubkey, account: &AccountInfo) -> Result<u8, ProgramError> {
    let path = &[crate::PREFIX.as_bytes(), program_id.as_ref(), "configure".as_bytes()];
    assert_derivation(&program_id, &account, path)
}

pub fn assert_creator_data(
    program_id: &Pubkey,
    creator: &AccountInfo,
    creator_data: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        creator.key.as_ref(),
        "creator_whitelist".as_bytes(),
    ];
    assert_derivation(&program_id, &creator_data, path)
}

pub fn assert_bid_data(
    program_id: &Pubkey,
    auction: &AccountInfo,
    bidder: &AccountInfo,
    bid_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        auction.key.as_ref(),
        bidder.key.as_ref(),
        "bid".as_bytes(),
    ];
    assert_derivation(&program_id, &bid_info, path)
}

pub fn assert_auction_authority(
    program_id: &Pubkey,
    auction_info: &AccountInfo,
    authority_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_info.key.as_ref(),
        "authority".as_bytes(),
    ];
    assert_derivation(&program_id, &authority_info, path)
}

pub fn assert_nft_store(
    program_id: &Pubkey,
    auction_info: &AccountInfo,
    nft_store_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        auction_info.key.as_ref(),
        "nft_store".as_bytes(),
    ];
    assert_derivation(&program_id, &nft_store_info, path)
}

pub fn assert_bid_store(
    program_id: &Pubkey,
    offer_info: &AccountInfo,
    bid_store_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        crate::PREFIX.as_bytes(),
        program_id.as_ref(),
        offer_info.key.as_ref(),
        "bid_store".as_bytes(),
    ];
    assert_derivation(&program_id, &bid_store_info, path)
}

pub fn assert_user_info(
    program_id: &Pubkey,
    user: &AccountInfo,
    user_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        user.key.as_ref(),
        "user_info".as_bytes(),
    ];
    assert_derivation(&program_id, &user_info, path)
}

pub fn assert_pool_info(
    program_id: &Pubkey,
    pool_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        "pool_info".as_bytes(),
    ];
    assert_derivation(&program_id, &pool_info, path)
}

pub fn assert_pool_authority(
    program_id: &Pubkey,
    pool_info: &AccountInfo,
    pool_authority: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        pool_info.key.as_ref(),
        "pool_authority".as_bytes(),
    ];
    assert_derivation(&program_id, &pool_authority, path)
}
