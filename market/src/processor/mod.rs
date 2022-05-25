use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod configure;
pub use configure::*;

pub mod set_creator_whitelist;
pub use set_creator_whitelist::*;

pub mod create;
pub use create::*;

pub mod place_bid;
pub use place_bid::*;

pub mod cancel;
pub use cancel::*;

pub mod change_price;
pub use change_price::*;

pub mod make_offer;
pub use make_offer::*;

pub mod cancel_offer;
pub use cancel_offer::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    match AppInstruction::try_from_slice(input)? {
        AppInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        AppInstruction::SetCreatorWhitelist(args) => {
            msg!("Instruction: SetCreatorWhitelist");
            process_set_creator_whitelist(program_id, accounts, args)
        }
        AppInstruction::Create(args) => {
            msg!("Instruction: Create");
            process_create(program_id, accounts, args)
        }
        AppInstruction::PlaceBid(args) => {
            msg!("Instruction: PlaceBid");
            process_place_bid(program_id, accounts, args)
        }
        AppInstruction::ChangePrice(args) => {
            msg!("Instruction: ChangePrice");
            process_change_price(program_id, accounts, args)
        }
        AppInstruction::Cancel => {
            msg!("Instruction: Cancel");
            process_cancel(program_id, accounts)
        }
        AppInstruction::MakeOffer(args) => {
            msg!("Instruction: MakeOffer");
            process_make_offer(program_id, accounts, args)
        }
        AppInstruction::CancelOffer => {
            msg!("Instruction: CancelOffer");
            process_cancel_offer(program_id, accounts)
        }
    }
}
