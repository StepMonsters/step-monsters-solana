use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct InitArgs {
    /// pool state.
    pub pool_state: bool,
    /// send ERC20 token to pool
    pub amount: u64,
    /// reward per second
    pub reward_per_second: u64,
}


