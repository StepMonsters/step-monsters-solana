use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};
use crate::utils_mint::{create_metadata_edition_create_collection};

pub fn process_create_collection(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _args: QuickMintArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let fee_receiver_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let _monster_info = next_account_info(account_info_iter)?;
    let _game_config_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    let config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    msg!("Create Metadata Edition");
    create_metadata_edition_create_collection(
        &program_id,
        &pda_creator_info,
        config_data.clone(),
        &signer_info,
        &mint_info,
        &metadata_info,
        &edition_info,
        &metadata_program_info,
        &token_program_info,
        &system_info,
        &rent_info,
    )?;

    Ok(())
}
