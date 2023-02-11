use borsh::BorshSerialize;
use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction, sysvar};

use crate::{ferror, state::*};
use crate::utils::{assert_config, assert_eq_pubkey, assert_signer, spl_token_transfer_invoke};
use crate::utils_config::calculate_synthesize_spend_game_token;
use crate::utils_mint::{create_metadata_edition, create_monster_info, spl_token_burn_quick};

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
    let _program_info = next_account_info(account_info_iter)?;
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

    let args = MintArgs {
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

    msg!("Assert Public Key");
    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_config(&program_id, &config_info)?;

    msg!("Assert Fee Receiver");
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    assert_eq_pubkey(&fee_receiver_info, &config_data.fee_receiver)?;

    msg!("Check Config Initialized");
    if !config_data.is_initialized {
        return ferror!("invalid mint state");
    }

    msg!("Transfer Mint Fee");
    if config_data.price > 0 {
        invoke(
            &system_instruction::transfer(&signer_info.key, &config_data.fee_receiver, config_data.price),
            &[
                signer_info.clone(),
                fee_receiver_info.clone(),
                system_info.clone(),
            ],
        )?;
    }

    msg!("Create Metadata Edition");
    create_metadata_edition(
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
    config_data.current_id += 1;
    config_data.serialize(&mut *config_info.try_borrow_mut_data()?)?;

    msg!("Create Monster Info");
    create_monster_info(
        &program_id,
        &monster_info,
        &mint_info,
        &rent_info,
        &system_info,
        &signer_info,
    )?;
    let mut monster = Monster::from_account_info(monster_info)?;
    monster.race = args.race;
    monster.generation = args.generation;
    monster.monster_feature = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    monster.father_mint = args.father_mint;
    monster.mother_mint = args.mother_mint;
    monster.serialize(&mut *monster_info.try_borrow_mut_data()?)?;

    Ok(())
}
