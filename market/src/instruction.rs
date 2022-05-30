use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::*;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::{rent},
};
// #[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    /// Setting auction global configuration
    Configure(ConfigureArgs),
    /// Set whitelist of nft creator, then those can call `Create`
    SetCreatorWhitelist(SetCreatorWhitelistArgs),
    /// Create fixed price sale or english auction
    Create(CreateArgs),
    /// Bid fixed price sale or english auction
    PlaceBid(PlaceBidArgs),
    ///change price
    ChangePrice(ChangePriceArgs),
    /// Cancel fixed price sale or english auction
    Cancel,
    ///make offer
    MakeOffer(MakeOfferArgs),
    ///cancel offer
    CancelOffer,
    ///accept offer
    AcceptOffer
}

pub fn configure(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    args: ConfigureArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn set_white(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    creator_info: &Pubkey,
    creator_data_info: &Pubkey,
    args: SetCreatorWhitelistArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*creator_info, false),
        AccountMeta::new(*creator_data_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::SetCreatorWhitelist(args)
            .try_to_vec()
            .unwrap(),
    })
}

pub fn create(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    creator_info: &Pubkey,
    creator_data_info: &Pubkey,
    new_auction_info: &Pubkey,
    authority_info: &Pubkey,
    nft_mint_info: &Pubkey,
    nft_metadata_info: &Pubkey,
    nft_account_info: &Pubkey,
    nft_store_info: &Pubkey,
    spl_token_info: &Pubkey,
    args: CreateArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*creator_info, false),
        AccountMeta::new(*creator_data_info, false),
        AccountMeta::new(*new_auction_info, true),
        AccountMeta::new(*authority_info, false),
        AccountMeta::new(*nft_mint_info, false),
        AccountMeta::new(*nft_metadata_info, false),
        AccountMeta::new(*nft_account_info, false),
        AccountMeta::new(*nft_store_info, false),
        AccountMeta::new_readonly(*spl_token_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Create(args).try_to_vec().unwrap(),
    })
}

pub fn create_change_price(
    program_id: &Pubkey,
    siger: &Pubkey,
    auction_info: &Pubkey,
    args: ChangePriceArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*auction_info, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::ChangePrice(args).try_to_vec().unwrap(),
    })
}

pub fn cancel(
    program_id: &Pubkey,
    siger: &Pubkey,
    auction_info: &Pubkey,
    authority_info: &Pubkey,
    nft_store_info: &Pubkey,
    nft_return_info: &Pubkey,
    spl_token_info: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*auction_info, false),
        AccountMeta::new(*authority_info, false),
        AccountMeta::new(*nft_store_info, false),
        AccountMeta::new(*nft_return_info, false),
        AccountMeta::new_readonly(*spl_token_info, false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Cancel.try_to_vec().unwrap(),
    })
}

pub fn buy(
    program_id: &Pubkey,
    siger: &Pubkey,
    charge_addr_info: &Pubkey,
    config_info: &Pubkey,
    auction_info: &Pubkey,
    authority_info: &Pubkey,
    bid_info: &Pubkey,
    auction_creator_info: &Pubkey,
    nft_store_info: &Pubkey,
    nft_return_info: &Pubkey,
    nft_metadata_info: &Pubkey,
    spl_token_info: &Pubkey,
    nft_creators: Vec<Pubkey>,
    args: PlaceBidArgs,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*charge_addr_info, false),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*auction_info, false),
        AccountMeta::new(*authority_info, false),
        AccountMeta::new(*bid_info, false),
        AccountMeta::new(*auction_creator_info, false),
        AccountMeta::new(*nft_store_info, false),
        AccountMeta::new(*nft_return_info, false),
        AccountMeta::new(*nft_metadata_info, false),
        AccountMeta::new_readonly(*spl_token_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    for creator in nft_creators.iter() {
        accounts.push(AccountMeta::new(*creator, false))
    }
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::PlaceBid(args).try_to_vec().unwrap(),
    })
}

pub fn create_make_offer(
    program_id: &Pubkey,
    siger: &Pubkey,
    nft_info: &Pubkey,
    nft_return_info: &Pubkey,
    new_offer_info: &Pubkey,
    bid_store_info: &Pubkey,
    args: MakeOfferArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*nft_info, false),
        AccountMeta::new(*nft_return_info, false),
        AccountMeta::new(*new_offer_info, false),
        AccountMeta::new(*bid_store_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::MakeOffer(args).try_to_vec().unwrap(),
    })
}

pub fn create_cancel_offer(
    program_id: &Pubkey,
    siger: &Pubkey,
    nft_info: &Pubkey,
    nft_return_info: &Pubkey,
    offer_info: &Pubkey,
    bid_store_info: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*nft_info, false),
        AccountMeta::new(*nft_return_info, false),
        AccountMeta::new(*offer_info, false),
        AccountMeta::new(*bid_store_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::CancelOffer.try_to_vec().unwrap(),
    })
}

pub fn create_accept_offer(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    charge_addr_info: &Pubkey,
    nft_account_info: &Pubkey,
    nft_return_info: &Pubkey,
    nft_metadata_info: &Pubkey,
    offerer_info: &Pubkey,
    new_offer_info: &Pubkey,
    bid_store_info: &Pubkey,
    spl_token_info: &Pubkey,
    nft_creators: Vec<Pubkey>,
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*charge_addr_info, false),
        AccountMeta::new(*nft_account_info, false),
        AccountMeta::new(*nft_return_info, false),
        AccountMeta::new(*nft_metadata_info, false),
        AccountMeta::new(*offerer_info, false),
        AccountMeta::new(*new_offer_info, false),
        AccountMeta::new(*bid_store_info, false),
        AccountMeta::new_readonly(*spl_token_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    for creator in nft_creators.iter() {
        accounts.push(AccountMeta::new(*creator, false))
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::AcceptOffer.try_to_vec().unwrap(),
    })
}