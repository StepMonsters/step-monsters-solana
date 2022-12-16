use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{state::*, utils::*};
use crate::utils_config::{get_monster_female_basic_attrs, get_monster_male_basic_attrs};

pub fn process_create_game_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;
    let game_config_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;

    msg!("Create Game Config");
    let bump_seed = assert_derivation(
        program_id,
        game_config_info,
        &[
            SEED_GAME_CONFIG.as_bytes(),
            program_id.as_ref(),
        ],
    )?;
    let game_config_seeds = &[
        SEED_GAME_CONFIG.as_bytes(),
        program_id.as_ref(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        game_config_info,
        rent_info,
        system_info,
        signer_info,
        MAX_GAME_CONFIG_LENGTH,
        game_config_seeds,
    )?;

    let male_data: [[u32; 6]; 10] = get_monster_male_basic_attrs();
    let female_data: [[u32; 6]; 10] = get_monster_female_basic_attrs();

    let male = game_config_to_vector(male_data);
    let female = game_config_to_vector(female_data);

    let mut game_config = GameConfig::from_account_info(game_config_info)?;
    game_config.monster_male = male;
    game_config.monster_female = female;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}

pub fn process_update_game_config(
    _: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_config_info = next_account_info(account_info_iter)?;

    let game_config = GameConfig::from_account_info(game_config_info)?;
    game_config.serialize(&mut *game_config_info.try_borrow_mut_data()?)?;

    Ok(())
}