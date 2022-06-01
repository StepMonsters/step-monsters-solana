use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// ======== ======== ======== ======== ======== ======== ======== ========

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureArgs {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// Charge rate (* 10000) of auction deal
    pub charge_rate: u64,
    /// Charge address with mint of WSOL
    pub charge_addr: Pubkey,
}

// pub type ConfigureData = ConfigureArgs;
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureData {
    /// Initialized state.
    pub is_initialized: bool,
    /// Contract admin
    pub authority: Pubkey,
    /// Charge rate (* 10000) of auction deal
    pub charge_rate: u64,
    /// Charge address with mint of WSOL
    pub charge_addr: Pubkey,
    /// total trade
    pub total_trade: u64,

}
impl ConfigureData {
    pub const LEN: usize = 1 + 32 + 8 + 32 + 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

// ======== ======== ======== ======== ======== ======== ======== ========

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct SetCreatorWhitelistArgs {
    /// Initialized state.
    pub is_activated: bool,
    pub total_supply: u64,
    pub seller_fee: u64,
    pub symbol: String,
}

pub type SetCreatorWhitelistData = SetCreatorWhitelistArgs;

impl SetCreatorWhitelistData {
    pub const LEN: usize = 1 + 8 + 8 + 64;

    pub fn from_account_info(a: &AccountInfo) -> Result<SetCreatorWhitelistData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

// ======== ======== ======== ======== ======== ======== ======== ========

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct CreateArgs {
    /// Price of sale
    pub price: Option<u64>,

    /// Auction duration, unix seconds
    pub duration: Option<u64>,

    /// Auction begin timestamp
    pub begin_ts: Option<u64>,
}



#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct AuctionData {
    pub timestamp: u64,
    pub is_init: bool,
    /// creator
    pub creator: Pubkey,
    /// NFT mint address
    pub nft_mint: Pubkey,
    /// NFT store by auction
    pub nft_store: Pubkey,
    /// fixed price sale
    /// Price of sale
    pub price: Option<u64>,
    /// Auction begin at unix timestamp
    pub begin_ts: Option<u64>,
    /// Auction duration, unix seconds
    pub duration: Option<u64>,
    /// fixed price    : be true after creator cancel or claim token
    pub is_claim: bool,
    /// Last bid
    pub last_bid: Option<BidData>,
}

impl AuctionData {
    pub const LEN: usize = 8 + 1 + 32 * 3 + 9 * 3 + 1 + (BidData::LEN + 1);

    pub fn from_account_info(a: &AccountInfo) -> Result<AuctionData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn is_initialized(&self) -> bool {
        return self.is_init
    
    }

}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct PlaceBidArgs {
    /// Price of bid
    pub price: u64,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct ChangePriceArgs {
    /// Price of bid
    pub price: u64,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct MakeOfferArgs {
    /// Price of bid
    pub price: u64,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct BidData {
    pub bidder: Pubkey,
    pub amount: u64,
    pub is_done: bool,
}

impl BidData {
    pub const LEN: usize = 32 + 8 + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<BidData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Copy, Debug, Default, PartialEq)]
pub struct OfferData {
    pub timestamp: u64,
    pub offerer: Pubkey,
    pub nft: Pubkey,
    pub nft_return: Pubkey,
    pub price: u64,
    pub is_canceled: bool,
    pub is_done: bool,
}

impl OfferData {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 1 + 1;

    pub fn from_account_info(a: &AccountInfo) -> Result<OfferData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}
