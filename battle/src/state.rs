use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};


// pub const MAX_MONSTER_LENGTH: usize = 1 * 4 + 4 * 5;
// pub const MAX_BATTLE_LENGTH: usize = 1;

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
// pub enum Key {
//     Uninitialized,
//     Monster,
//     Battle,
// }

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
// pub struct BattleArgs {
//     pub hp: u32,
//     pub defense: u32,
//     pub attact: u32,
// }

// #[repr(C)]
// #[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
// pub struct Battle {
//     pub winner: u8,
// }

// impl Battle {
//     pub fn from_account_info(a: &AccountInfo) -> Result<Battle, ProgramError> {
//         if a.data_len() != MAX_BATTLE_LENGTH {
//             return Err(ProgramError::InvalidAccountData);
//         }
//         let battle: Battle =
//             try_from_slice_unchecked(&a.data.borrow_mut())?;
//         Ok(battle)
//     }
// }





