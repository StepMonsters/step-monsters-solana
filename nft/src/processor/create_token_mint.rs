use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{utils::*};
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

    let token_program_info = next_account_info(account_info_iter)?;
    let ass_token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let decimal: u64 = 1_000_000_000;
    let amount: u64 = 10 * decimal;
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

    Ok(())
}
