use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};

use crate::{utils::*};
use crate::instruction::mint;

pub fn process_breed(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let fee_receiver_info = next_account_info(account_info_iter)?; // fee_receiver: wallet
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // let father_info = next_account_info(account_info_iter)?;
    // let mother_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    // let size = 82;
    // let rent = &Rent::from_account_info(&rent_info)?;
    // let required_lamports = rent.minimum_balance(size);

    // let father = Monster::from_account_info(father_info)?;
    // let mother = Monster::from_account_info(mother_info)?;

    invoke(
        &mint(
            program_id,
            signer_info.key,
            config_info.key,
            pda_creator_info.key,
            fee_receiver_info.key,
            mint_info.key,
            metadata_info.key,
            edition_info.key,
            monster_info.key,
            metadata_program_info.key,
            token_program_info.key,
        )?,
        &[
            signer_info.clone(),
            config_info.clone(),
            pda_creator_info.clone(),
            fee_receiver_info.clone(),
            mint_info.clone(),
            metadata_info.clone(),
            edition_info.clone(),
            monster_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            rent_info.clone(),
            system_info.clone()
        ],
    )?;

    Ok(())
}
