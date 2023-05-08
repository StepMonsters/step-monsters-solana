use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use solana_program::program::invoke_signed;

use crate::{ferror, utils::*};
use crate::state::{ConfigureData, SEED_TOKEN_ADMIN};
use crate::utils_mint::mint_game_token_to_ata;

pub fn process_create_token_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;

    let token_admin_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;

    let metadata_info = next_account_info(account_info_iter)?;
    
    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    //check authority
    let config_info = next_account_info(account_info_iter)?;
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }

    assert_signer(&signer_info)?;

    let decimal: u64 = 1_000_000_000;
    let amount: u64 = 20000 * decimal;
    mint_game_token_to_ata(
        program_id,
        signer_info,
        signer_ata_info,
        mint_info,
        token_admin_info,
        ass_token_program_info,
        token_program_info,
        system_info,
        amount,
    )?;

    msg!("Token Admin Seeds");
    let bump_seed = assert_derivation(
        program_id,
        token_admin_info,
        &[
            SEED_TOKEN_ADMIN.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let token_admin_seeds = [
        SEED_TOKEN_ADMIN.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];

    let name = String::from("Lite Satoshi Token");
    let symbol = String::from("LST");
    let uri = String::from("https://api.stepmonsters.xyz/metadata/lst.json");

    msg!("Create Metadata");
    invoke_signed(
        &create_metadata_accounts_v3(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *token_admin_info.key,
            *signer_info.key,
            *token_admin_info.key,
            name.clone(),
            symbol.clone(),
            uri.clone(),
            None,
            0,
            true,
            true,
            None,
            None,
            None
        ),
        &[
            metadata_program_info.clone(),
            metadata_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            token_admin_info.clone(),
            system_info.clone(),
            token_program_info.clone(),
            rent_info.clone(),
        ],
        &[&token_admin_seeds],
    )?;

    Ok(())
}
