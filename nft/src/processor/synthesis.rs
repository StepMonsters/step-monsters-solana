use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*};
use crate::instruction::mint;
use crate::utils::{assert_signer, spl_token_transfer_invoke};
use crate::utils_config::calculate_synthesize_spend_game_token;
use crate::utils_mint::spl_token_burn_quick;

pub fn process_synthesis(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let fee_receiver_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let monster_info = next_account_info(account_info_iter)?;
    let program_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let program_ata_info = next_account_info(account_info_iter)?;

    let token_account_01 = next_account_info(account_info_iter)?;
    let token_account_02 = next_account_info(account_info_iter)?;
    let mint_info_01 = next_account_info(account_info_iter)?;
    let mint_info_02 = next_account_info(account_info_iter)?;
    let monster_info_01 = next_account_info(account_info_iter)?;
    let monster_info_02 = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    let monster_01 = Monster::from_account_info(monster_info_01)?;
    let monster_02 = Monster::from_account_info(monster_info_02)?;

    let new_race;

    if monster_01.race == 0 && monster_02.race == 1 {
        new_race = 4;
    } else if monster_01.race == 1 && monster_02.race == 0 {
        new_race = 4;
    } else {
        return ferror!("wrong race");
    }

    msg!("Synthesize LST spending");
    let spend = calculate_synthesize_spend_game_token(monster_01.race, monster_02.race);
    spl_token_transfer_invoke(
        token_program_info.clone(),
        signer_ata_info.clone(),
        program_ata_info.clone(),
        signer_info.clone(),
        spend,
    )?;

    let mint_args = MintArgs {
        race: new_race,
        attrs: Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        generation: 1,
        father_mint: *system_info.key,
        mother_mint: *system_info.key,
    };

    msg!("Burn Token");
    spl_token_burn_quick(
        mint_info_01.clone(),
        signer_info.clone(),
        token_program_info.clone(),
        token_account_01.clone(),
    )?;

    msg!("Burn Token");
    spl_token_burn_quick(
        mint_info_02.clone(),
        signer_info.clone(),
        token_program_info.clone(),
        token_account_02.clone(),
    )?;

    msg!("Mint New Token");
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
            mint_args,
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
            program_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            rent_info.clone(),
            system_info.clone()
        ],
    )?;

    Ok(())
}
